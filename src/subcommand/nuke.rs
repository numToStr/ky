use std::fs::remove_dir_all;

use clap::Clap;

use crate::{
    cli::Config,
    lib::{Database, KyError, Password, Prompt, MASTER},
};

use super::Command;

#[derive(Debug, Clap)]
pub struct Nuke;

impl Command for Nuke {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let db = Database::new(&config.db_path())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let db_path = config.db_path();

        if db_path.exists() && Prompt::proceed(&theme)? {
            remove_dir_all(db_path)?;
        }

        println!("Vault successfully nuked");

        Ok(())
    }
}
