use crate::{
    check_db,
    cli::Config,
    lib::{Cipher, Encrypted, KyEnv, KyError, KyResult, KyTable, Password, Prompt, MASTER},
};
use clap::Clap;

use super::Command;

#[derive(Debug, Clap)]
pub struct Ls;

impl Command for Ls {
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let master_pwd = Password::ask_master(&Prompt::theme())?;

        let env = KyEnv::connect(&db_path)?;

        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        let keys = pwd_db.ls(&rtxn)?;

        rtxn.commit()?;

        println!();
        if keys.is_empty() {
            println!("> No entries found!");
        } else {
            let key_cipher = Cipher::for_key(&master_pwd);

            for (key, _) in keys {
                let key = key_cipher.decrypt(&key)?;
                println!("- {}", String::from(key));
            }
        }

        env.close();

        Ok(())
    }
}
