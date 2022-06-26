use super::Command;
use crate::{
    cli::Config,
    echo,
    lib::{entity::Master, KyDb2, KyError, KyResult, KyTable, Prompt},
};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Init;

impl Command for Init {
    fn exec(self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        if db_path.exists() {
            return Err(KyError::Init);
        }

        let hashed = Master::confirm(&Prompt::theme())?.hash()?;

        let db = KyDb2::new(&db_path)?;
        let wtxn = db.wtxn()?;

        {
            let mut tbl = db.open_write(&wtxn, KyTable::Master)?;
            tbl.set(Master::KEY.into(), hashed)?;
        }

        wtxn.commit()?;

        echo!("> Vault Initiliazed!");

        Ok(())
    }
}
