use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{Database, KyError, Password, Prompt, MASTER},
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

        let password = Password::init(&Prompt::theme())?;

        let hashed = password.hash()?;

        let db = Database::open(config.ensure_create(&db_path))?;

        let mut txn = db.write_txn()?;

        db.set(&mut txn, MASTER, &hashed)?;

        txn.commit()?;

        echo!("> Vault Initiliazed!");

        db.close();

        Ok(())
    }
}
