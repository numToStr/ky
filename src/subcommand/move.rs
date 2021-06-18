use super::Command;
use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{Cipher, Database, KyError, Password, Prompt, Value, MASTER},
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

        let db = Database::open(&db_path)?;

        let rtxn = db.read_txn()?;

        let hashed = db.get(&rtxn, MASTER)?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        // first check if the old key exist or not
        // If exist, then retrieve the value
        let encrypted = db.get(&rtxn, &self.old_key)?;

        // now check if the new key exists or not
        if db.get(&rtxn, &self.new_key).is_ok() {
            return Err(KyError::Exist(self.new_key.to_string()));
        }

        rtxn.commit()?;

        echo!("- Decrypting old details...");
        let old_cipher = Cipher::new(&master_pwd.to_string(), &self.old_key);

        let old_val = Value::decrypt(&old_cipher, &encrypted)?;

        println!("- Encrypting new details...");
        let new_cipher = Cipher::new(&master_pwd.to_string(), &self.new_key);
        let new_val = Value {
            password: old_cipher.decrypt(&old_val.password)?,
            username: old_val.username,
            website: old_val.website,
            expires: old_val.expires,
            notes: old_val.notes,
        }
        .encrypt(&new_cipher)?;

        let mut wtxn = db.write_txn()?;

        db.set(&mut wtxn, &self.new_key, &new_val)?;
        db.delete(&mut wtxn, &self.old_key)?;

        wtxn.commit()?;

        db.close();

        echo!(
            "> Entry moved: {} -> {}",
            style(&self.old_key).bold(),
            style(&self.new_key).bold()
        );

        Ok(())
    }
}
