use super::Command;
use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{entity::Master, Encrypted, KyEnv, KyError, KyResult, KyTable, Prompt, Vault, MASTER},
};
use clap::Clap;
use dialoguer::console::style;
use std::path::PathBuf;

#[derive(Debug, Clap)]
pub struct Backup {
    /// Path to the backup file
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore and delete existing backup file
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for Backup {
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master = Master::ask(&theme)?;

        let env = KyEnv::connect(&db_path)?;
        let common_db = env.get_table(KyTable::Common)?;

        let rtxn = env.read_txn()?;

        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        rtxn.commit()?;

        env.close();

        if !master.verify(hashed.as_ref())? {
            return Err(KyError::MisMatch);
        }

        let backup_path = match &self.path {
            Some(p) => p.to_path_buf(),
            _ => config.backup_path(),
        };

        // ignore flag is set
        // vault backup is already exists
        // then ask to proceed
        if !self.ignore && backup_path.exists() && !Prompt::backup_exist(&theme)? {
            return Ok(());
        }

        Vault::new(&db_path).backup(&backup_path)?;

        echo!("> Vault backed-up: {}", style(backup_path.display()).bold());

        Ok(())
    }
}
