use crate::{
    check_db,
    cli::Config,
    lib::{Cipher, KyEnv, KyError, KyTable, Password, Prompt, MASTER},
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

        let env = KyEnv::connect(&db_path)?;

        let master_db = env.get_table(KyTable::Master)?;
        let password_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = master_db.get(&rtxn, MASTER)?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        let keys = password_db.ls(&rtxn)?;

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

        env.close();

        Ok(())
    }
}
