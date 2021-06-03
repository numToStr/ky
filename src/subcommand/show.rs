use crate::{
    cli::Config,
    lib::{Cipher, Database, KyError, Password},
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

        // The crypted data returned from database
        // Will be in this format password:username:url:expires:notes
        let data = db.get(&self.key)?;
        let crypted = data.splitn(5, ':');

        let cipher = Cipher::new(&master_pwd.to_string());

        // let password = crypted.next();
        // let username = crypted.next();
        // let url = crypted.next();
        // let expires = crypted.next();
        // let notes = crypted.next();

        for c in crypted {
            match c {
                "-" => {
                    println!("Empty");
                }
                x => {
                    let decrypted = cipher.decrypt(x)?;
                    println!("{}", decrypted);
                }
            }
        }

        Ok(())
    }
}
