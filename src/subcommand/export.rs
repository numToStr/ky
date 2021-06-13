use std::path::PathBuf;

use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{Database, KyError, Password, Prompt, Vault, MASTER},
};
use clap::Clap;
use dialoguer::console::style;

use super::Command;

#[derive(Debug, Clap)]
pub struct Export {
    /// Path to the export file
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore and delete existing exported file
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for Export {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();

        let master_pwd = Password::ask_master(&theme)?;

        let db = Database::open(&db_path)?;

        let rtxn = db.read_txn()?;

        let hashed = db.get(&rtxn, MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let keys = db.ls(&rtxn)?;

        rtxn.commit()?;

        let export_path = match &self.path {
            Some(p) => p.to_path_buf(),
            _ => config.csv_path(),
        };

        if !self.ignore && export_path.exists() && !Prompt::export_exist(&theme)? {
            return Ok(());
        }

        Vault::export(&master_pwd.to_string(), keys, &export_path)?;

        echo!("> Vault exported: {}", style(export_path.display()).bold());

        Ok(())
    }
}
