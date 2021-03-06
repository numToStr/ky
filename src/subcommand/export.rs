use std::path::PathBuf;

use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{entity::Master, Encrypted, KyEnv, KyError, KyResult, KyTable, Prompt, Vault, MASTER},
};
use clap::Parser;
use dialoguer::console::style;

use super::Command;

#[derive(Debug, Parser)]
pub struct Export {
    /// Path to the export file
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore and delete existing exported file
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for Export {
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();

        let master = Master::ask(&theme)?;

        let env = KyEnv::connect(&db_path)?;
        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        if !master.verify(hashed.as_ref())? {
            return Err(KyError::MisMatch);
        }

        let keys = pwd_db.ls(&rtxn)?;

        rtxn.commit()?;

        env.close();

        let export_path = match &self.path {
            Some(p) => p.to_path_buf(),
            _ => config.export_path(),
        };

        if !self.ignore && export_path.exists() && !Prompt::export_exist(&theme)? {
            return Ok(());
        }

        Vault::export(&export_path, &master, keys)?;

        echo!("> Vault exported: {}", style(export_path.display()).bold());

        Ok(())
    }
}
