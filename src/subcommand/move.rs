use super::Command;
use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{
        Cipher, Details, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable, Password, Prompt,
        MASTER,
    },
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Move {
    /// Current name of the key
    old_key: EntryKey,

    /// New name for the key
    new_key: EntryKey,
}

impl Command for Move {
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

        let key_cipher = Cipher::for_key(&master_pwd);

        // first check if the old key exist or not
        // If exist, then retrieve the value
        let old_key = key_cipher.encrypt(&self.old_key.as_ref())?;
        let encrypted = pwd_db.get(&rtxn, &old_key)?;

        // now check if the new key exists or not
        let new_key = key_cipher.encrypt(&self.new_key.as_ref())?;
        if pwd_db.get(&rtxn, &new_key).is_ok() {
            return Err(KyError::Exist);
        }

        rtxn.commit()?;

        echo!("- Decrypting old details...");
        let old_cipher = Cipher::for_value(&master_pwd, &self.old_key)?;

        let old_val = Details::decrypt(&old_cipher, &encrypted)?;

        println!("- Encrypting new details...");
        let new_cipher = Cipher::for_value(&master_pwd, &self.new_key)?;
        let new_val = Details {
            password: old_cipher.decrypt(&old_val.password)?.into(),
            username: old_val.username,
            website: old_val.website,
            expires: old_val.expires,
            notes: old_val.notes,
        }
        .encrypt(&new_cipher)?;

        let mut wtxn = env.write_txn()?;

        pwd_db.set(&mut wtxn, &new_key, &new_val)?;
        pwd_db.delete(&mut wtxn, &old_key)?;

        wtxn.commit()?;

        env.close();

        echo!(
            "> Entry moved: {} -> {}",
            style(&self.old_key.as_ref()).bold(),
            style(&self.new_key.as_ref()).bold()
        );

        Ok(())
    }
}
