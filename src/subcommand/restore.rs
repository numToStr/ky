use std::path::PathBuf;

use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{KyError, KyResult, Prompt, Vault},
};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Restore {
    /// Path to the vault backup
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore already initialized vault, if any
    #[clap(short, long)]
    ignore: bool,
}

impl Command for Restore {
    fn exec(self, config: Config) -> KyResult<()> {
        let theme = Prompt::theme();
        // let master_pwd = Password::ask_master(&theme)?;

        let backup_path = match self.path {
            Some(x) => x,
            _ => config.backup_path(),
        };

        if !backup_path.exists() {
            return Err(KyError::BackupDontExist);
        }

        if !self.ignore && config.db_path().exists() && !Prompt::vault_exist(&theme)? {
            return Ok(());
        }

        Vault::restore(&backup_path, &config.vault_path())?;

        echo!("> Vault restored!");

        Ok(())
    }
}
