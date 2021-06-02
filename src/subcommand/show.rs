use crate::{
    cli::Config,
    lib::{Database, Encrypt, KyError, Password},
};
use clap::Clap;

use super::{Command, MASTER};

#[derive(Debug, Clap)]
pub struct Show {
    /// Unique key for the entry
    key: String,
}

impl Command for Show {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let master_pwd = Password::ask_master()?;

        let db = Database::new(config.db_path())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let crypted = db.get(&self.key)?;

        let enc_key = master_pwd.to_string();
        let decrypted = Encrypt::new(&enc_key).decrypt(&crypted).unwrap();

        println!("{}", decrypted);

        Ok(())
    }
}
