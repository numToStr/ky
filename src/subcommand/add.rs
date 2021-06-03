use crate::{
    cli::{Config, PwdGenOpts},
    lib::{Cipher, Database, KyError, Password, Prompt},
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
        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let db = Database::new(config.db_path())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let username = Prompt::username(&theme)?;
        let url = Prompt::url(&theme)?;
        let expires = Prompt::expires(&theme)?;
        let notes = Prompt::notes(&theme)?;

        let enc_key = master_pwd.to_string();
        let cipher = Cipher::new(&enc_key);

        let new_pass = Password::generate(&self.pwd_opt).to_string();
        let pwd = cipher.encrypt(&new_pass)?;

        let fields = [username, url, expires, notes];

        // I know that there can be only 5 data fields
        let mut enc_data: Vec<String> = Vec::with_capacity(5);

        enc_data.push(pwd);

        for data in fields.iter() {
            if let Some(d) = data {
                let enc = cipher.encrypt(&d)?;
                enc_data.push(enc);
            } else {
                enc_data.push("-".to_string());
            }
        }

        db.set(&self.key, &enc_data.join(":"))?;

        Ok(())
    }
}
