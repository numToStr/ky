use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{Database2, KyError, Password, Prompt, MASTER},
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Init;

impl Command for Init {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        if db_path.exists() {
            return Err(KyError::Init);
        }

        let env = Database2::env(config.ensure_create(&db_path))?;
        let txn = env.begin_rw_txn()?;

        let db = Database2::open(&txn)?;

        let password = Password::init(&Prompt::theme())?;

        let hashed = password.hash()?;

        db.set(MASTER, &hashed)?;

        txn.commit()?;

        echo!("> Vault Initiliazed!");

        Ok(())
    }
}
