use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{entity::Master, KyDb2, KyError, KyResult, KyTable, Prompt, Vault},
};
use clap::Parser;
use dialoguer::console::style;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Backup {
    /// Path to the backup file
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore and delete existing backup file
    #[clap(short, long)]
    ignore: bool,
}

impl Command for Backup {
    fn exec(self, config: Config) -> KyResult<()> {
        let theme = Prompt::theme();
        let master = Master::ask(&theme)?;

        let db_path = config.db_path();
        let db = KyDb2::connect(&db_path)?;

        {
            let rtxn = db.rtxn()?;
            let master_tbl = db.open_read(&rtxn, KyTable::Master)?;
            let hashed = master_tbl.get(&Master::KEY.into())?;

            if !master.verify(hashed)? {
                return Err(KyError::MisMatch);
            }
        }

        let backup_path = match self.path {
            Some(p) => p,
            _ => config.backup_path(),
        };

        // ignore flag is set
        // vault backup is already exists
        // then ask to proceed
        if !self.ignore && backup_path.exists() && !Prompt::backup_exist(&theme)? {
            return Ok(());
        }

        Vault::backup(&config.vault_path(), &backup_path)?;

        echo!("> Vault backed-up: {}", style(backup_path.display()).bold());

        Ok(())
    }
}
