use super::Command;
use crate::{
    check_db, check_decrypt, check_encrypt,
    cli::Config,
    echo,
    lib::{Cipher, Database2, KyError, Password, Prompt, Value, MASTER},
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Move {
    /// Current name of the key
    old_key: String,

    /// New name for the key
    new_key: String,
}

impl Command for Move {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let master_pwd = Password::ask_master(&Prompt::theme())?;

        let env = Database2::env(&db_path)?;
        let txn = env.begin_rw_txn()?;
        let db = Database2::open(&txn)?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        // first check if the old key exist or not
        // If exist, then retrieve the value
        let value = db.get(&self.old_key)?;

        // now check if the new key exists or not
        if db.get(&self.new_key).is_ok() {
            return Err(KyError::Exist(self.new_key.to_string()));
        }

        echo!("- Decrypting old details...");
        let old_value = Value::from(value.as_ref());
        let old_cipher = Cipher::new(&master_pwd.to_string(), &self.old_key);

        let old_username = check_decrypt!(old_cipher, &old_value.username);
        let old_password = check_decrypt!(old_cipher, &old_value.password);
        let old_url = check_decrypt!(old_cipher, &old_value.url);
        let old_expires = check_decrypt!(old_cipher, &old_value.expires);
        let old_notes = check_decrypt!(old_cipher, &old_value.notes);

        println!("- Encrypting new details...");
        let new_cipher = Cipher::new(&master_pwd.to_string(), &self.new_key);
        let new_value = Value {
            username: check_encrypt!(new_cipher, Some(old_username)),
            password: check_encrypt!(new_cipher, Some(old_password)),
            url: check_encrypt!(new_cipher, Some(old_url)),
            expires: check_encrypt!(new_cipher, Some(old_expires)),
            notes: check_encrypt!(new_cipher, Some(old_notes)),
        };

        db.set(&self.new_key, &new_value.to_string())?;
        db.delete(&self.old_key, &value)?;

        txn.commit()?;

        echo!(
            "> Entry moved: {} -> {}",
            style(&self.old_key).bold(),
            style(&self.new_key).bold()
        );

        Ok(())
    }
}
