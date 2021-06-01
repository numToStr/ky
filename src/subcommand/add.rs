use crate::{
    cli::Config,
    lib::{Database, Encrypt, KyError, Password},
};
use clap::Clap;

use super::{Command, MASTER};

#[derive(Debug, Clap)]
pub struct Add {
    /// Unique key for the entry
    key: String,

    /// Length of the password
    #[clap(short, long, default_value = "20")]
    length: u64,
}

impl Command for Add {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let master_pwd = Password::ask_master()?;

        let db = Database::new(config.db_path())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let new_pass = Password::generate(self.length);

        let enc_key = master_pwd.to_string();
        let enc_data = new_pass.to_string();
        let enc = Encrypt::new(&enc_key, &enc_data);

        let (pwd, nonce) = enc.encrypt()?;

        db.create_entry(&self.key, &pwd, &nonce)?;

        Ok(())
    }
}
