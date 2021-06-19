use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{key::EntryKey, Cipher, Database, Details, KyError, Password, Prompt, MASTER},
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Add {
    /// Unique key for the entry
    key: EntryKey,

    #[clap(flatten)]
    pwd_opt: PasswordParams,
}

impl Command for Add {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let db = Database::open(&db_path)?;

        let rtxn = db.read_txn()?;
        let hashed = db.get(&rtxn, MASTER)?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        let key = Cipher::for_key(&master_pwd).encrypt(&self.key.as_ref())?;

        if db.get(&rtxn, &key).is_ok() {
            return Err(KyError::Exist(self.key.as_ref().to_string()));
        }

        rtxn.commit()?;

        let username = Prompt::username(&theme)?;
        let website = Prompt::website(&theme)?;
        let expires = Prompt::expires(&theme)?;
        let notes = Prompt::notes(&theme)?;

        let cipher = Cipher::for_value(&master_pwd, &self.key)?;

        let new_pass = Password::generate(&self.pwd_opt).to_string();

        let encrypted = Details {
            password: new_pass,
            username,
            website,
            expires,
            notes,
        }
        .encrypt(&cipher)?;

        let mut wtxn = db.write_txn()?;

        db.set(&mut wtxn, &key, &encrypted)?;

        wtxn.commit()?;

        db.close();

        echo!("> Entry added: {}", style(&self.key.as_ref()).bold());

        Ok(())
    }
}
