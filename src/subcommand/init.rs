use super::Command;
use crate::{
    cli::Config,
    lib::{Database, KyError, Password, Prompt, MASTER},
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Init;

impl Command for Init {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db = Database::new(config.db_path())?;

        if db.exist(MASTER)? {
            return Err(KyError::Initialized);
        }

        let password = Password::init(&Prompt::theme())?;

        let hashed = password.hash()?;

        db.set(MASTER, &hashed)?;

        Ok(())
    }
}
