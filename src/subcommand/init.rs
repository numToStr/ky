use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{Encrypted, KyEnv, KyError, KyResult, KyTable, Password, Prompt, MASTER},
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Init;

impl Command for Init {
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        if db_path.exists() {
            return Err(KyError::Init);
        }

        let password = Password::init(&Prompt::theme())?;

        let hashed = password.hash()?;

        let env = KyEnv::connect(config.ensure_create(&db_path))?;

        let common_db = env.get_table(KyTable::Common)?;

        let mut txn = env.write_txn()?;

        common_db.set(&mut txn, &Encrypted::from(MASTER), &hashed)?;

        txn.commit()?;

        echo!("> Vault Initiliazed!");

        env.close();

        Ok(())
    }
}
