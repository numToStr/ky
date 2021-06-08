use std::{fs::remove_dir_all, path::PathBuf};

use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{KyError, Prompt, Vault},
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Restore {
    /// Path to the vault backup
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore already initialized vault, if any
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for Restore {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let theme = Prompt::theme();
        // let master_pwd = Password::ask_master(&theme)?;

        let backup_path = match &self.path {
            Some(x) => x.to_path_buf(),
            _ => config.backup_path(),
        };

        if !backup_path.exists() {
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

        Vault::new(&backup_path).restore(&db_path)?;

        echo!("> Vault restored!");

        Ok(())
    }
}
