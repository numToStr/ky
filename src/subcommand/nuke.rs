use std::fs::remove_dir_all;

use clap::Clap;

use crate::{
    check_db,
    cli::Config,
    lib::{Database, KyError, Password, Prompt, MASTER},
};

use super::Command;

#[derive(Debug, Clap)]
pub struct Nuke;

impl Command for Nuke {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let db = Database::open(&db_path)?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        if Prompt::proceed(&theme)? {
            remove_dir_all(db_path)?;
        }

        println!("Vault successfully nuked");

        Ok(())
    }
}
