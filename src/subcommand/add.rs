use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{Cipher, Database, KyError, Password, Prompt, Values, MASTER},
};
use clap::Clap;
use dialoguer::console::style;

#[macro_export]
macro_rules! check_encrypt {
    ($cipher: expr, $raw: expr) => {{
        use crate::lib::EMPTY;

        match $raw {
            Some(x) if x != EMPTY && x != "" => Some($cipher.encrypt(&x)?),
            _ => None,
        }
    }};
}

#[derive(Debug, Clap)]
pub struct Add {
    /// Unique key for the entry
    key: String,

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

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        if db.get(&rtxn, &self.key).is_ok() {
            return Err(KyError::Exist(self.key.to_string()));
        }

        rtxn.commit()?;

        let username = Prompt::username(&theme)?;
        let website = Prompt::website(&theme)?;
        let expires = Prompt::expires(&theme)?;
        let notes = Prompt::notes(&theme)?;

        let enc_key = master_pwd.to_string();
        let cipher = Cipher::new(&enc_key, &self.key);

        let new_pass = Password::generate(&self.pwd_opt).to_string();

        let val = Values {
            password: Some(cipher.encrypt(&new_pass)?),
            username: check_encrypt!(cipher, username),
            website: check_encrypt!(cipher, website),
            expires: check_encrypt!(cipher, expires),
            notes: check_encrypt!(cipher, notes),
        };

        let mut wtxn = db.write_txn()?;
        db.set(&mut wtxn, &self.key, &val.to_string())?;
        wtxn.commit()?;

        db.close();

        echo!("> Entry added: {}", style(&self.key).bold());

        Ok(())
    }
}
