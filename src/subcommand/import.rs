use crate::{
    cli::Config,
    echo,
    lib::{Database, KyError, Password, Prompt, Vault},
};
use clap::Clap;

use std::{fs::remove_dir_all, path::PathBuf};

use super::Command;

#[derive(Debug, Clap)]
pub struct Import {
    /// Path to the exported file
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore already initialized vault, if any
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for Import {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let theme = Prompt::theme();

        let master_pwd = Password::init(&theme)?;

        let import_path = match &self.path {
            Some(p) => p.to_path_buf(),
            _ => config.csv_path(),
        };

        if !import_path.exists() {
            return Err(KyError::BackupDontExist);
        }

        let db_path = config.db_path();
        let db_exist = db_path.exists();

        if !self.ignore && db_exist && !Prompt::vault_exist(&theme)? {
            return Ok(());
        }

        if db_exist {
            remove_dir_all(&db_path)?;
        }

        let db = Database::open(config.ensure_create(&db_path))?;

        Vault::import(&import_path, master_pwd, &db)?;

        db.close();

        echo!("> Vault imported!");

        Ok(())
    }
}
