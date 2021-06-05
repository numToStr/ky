use crate::{
    cli::Config,
    lib::{Database, KyError, Password, Prompt, MASTER},
};
use clap::Clap;

use super::Command;

#[derive(Debug, Clap)]
pub struct Ls;

impl Command for Ls {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db = Database::new(config.db_path())?;

        let master_pwd = Password::ask_master(&Prompt::theme())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        for key in db.ls() {
            println!("{}", key);
        }

        Ok(())
    }
}
