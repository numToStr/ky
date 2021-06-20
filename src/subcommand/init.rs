use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{KyEnv, KyError, KyTable, Password, Prompt, MASTER},
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

        let env = KyEnv::connect(config.ensure_create(&db_path))?;

        let master_db = env.get_table(KyTable::Master)?;

        let mut txn = env.write_txn()?;

        master_db.set(&mut txn, MASTER, &hashed)?;

        txn.commit()?;

        echo!("> Vault Initiliazed!");

        env.close();

        Ok(())
    }
}
