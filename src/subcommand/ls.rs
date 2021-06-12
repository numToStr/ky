use crate::{
    check_db,
    cli::Config,
    lib::{Database2, KyError, Password, Prompt, MASTER},
};
use clap::Clap;

use super::Command;

#[derive(Debug, Clap)]
pub struct Ls;

impl Command for Ls {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let master_pwd = Password::ask_master(&Prompt::theme())?;

        let env = Database2::env(&db_path)?;

        let txn = env.begin_rw_txn()?;

        let db = Database2::open(&txn)?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let keys = db.ls()?;

        println!();
        if keys.is_empty() {
            println!("> No entries found!");
        } else {
            for key in keys {
                println!("- {}", key);
            }
        }

        txn.commit()?;

        Ok(())
    }
}
