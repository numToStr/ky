use crate::{
    check_db,
    cli::Config,
    lib::{Cipher, Database, KyError, Password, Prompt, MASTER},
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

        let db = Database::open(&db_path)?;

        let rtxn = db.read_txn()?;

        let hashed = db.get(&rtxn, MASTER)?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        let keys = db.ls(&rtxn)?;

        rtxn.commit()?;

        println!();
        if keys.is_empty() {
            println!("> No entries found!");
        } else {
            let key_cipher = Cipher::for_key(&master_pwd);

            for (key, _) in keys {
                let key = key_cipher.decrypt(&key)?;
                println!("- {}", key);
            }
        }

        db.close();

        Ok(())
    }
}
