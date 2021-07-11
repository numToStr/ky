use crate::{
    check_db,
    cli::Config,
    lib::{entity::Master, Cipher, Encrypted, KyEnv, KyError, KyResult, KyTable, Prompt, MASTER},
};
use clap::Clap;

use super::Command;

#[derive(Debug, Clap)]
pub struct Ls;

impl Command for Ls {
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let master = Master::ask(&Prompt::theme())?;

        let env = KyEnv::connect(&db_path)?;

        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        if !master.verify(hashed.as_ref())? {
            return Err(KyError::MisMatch);
        }

        let keys = pwd_db.ls(&rtxn)?;

        rtxn.commit()?;

        println!();
        if keys.is_empty() {
            println!("> No entries found!");
        } else {
            let key_cipher = Cipher::for_key(&master);

            for (key, _) in keys {
                let key = key_cipher.decrypt(&key)?;
                println!("- {}", String::from(key));
            }
        }

        env.close();

        Ok(())
    }
}
