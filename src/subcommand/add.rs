use crate::{
    cli::{Config, PwdGenOpts},
    lib::{Database, Encrypt, KyError, Password},
};
use clap::Clap;

use super::{Command, MASTER};

#[derive(Debug, Clap)]
pub struct Add {
    /// Unique key for the entry
    key: String,

    #[clap(flatten)]
    pwd_opt: PwdGenOpts,
}

impl Command for Add {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let master_pwd = Password::ask_master()?;

        let db = Database::new(config.db_path())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let new_pass = Password::generate(self.pwd_opt.length);

        let enc_key = master_pwd.to_string();
        let enc_data = new_pass.to_string();

        let pwd = Encrypt::new(&enc_key).encrypt(&enc_data)?;

        db.set(&self.key, &pwd)?;

        Ok(())
    }
}
