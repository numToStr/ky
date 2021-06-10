use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{Database2, KyError, Password, Prompt, MASTER},
};
use clap::Clap;
use std::fs::remove_dir_all;

use super::Command;

#[derive(Debug, Clap)]
pub struct Nuke;

impl Command for Nuke {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let db = Database2::open(&db_path)?;

        let rtxn = db.read_txn()?;
        let hashed = db.get(&rtxn, MASTER)?;
        rtxn.commit()?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        if Prompt::proceed(&theme)? {
            remove_dir_all(db_path)?;
        }

        echo!("> Vault nuked!");

        Ok(())
    }
}
