use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{Cipher, Database, KyError, Password, Prompt, Value, MASTER, PREFIX},
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Edit {
    /// Entry which needs to be edited
    key: String,

    /// Allow password to be regenerated
    #[clap(short, long)]
    password: bool,

    #[clap(flatten)]
    pwd_opt: PasswordParams,
}

impl Command for Edit {
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

        let encrypted = db.get(&rtxn, &self.key)?;

        rtxn.commit()?;

        echo!(
            "  {}",
            style("Type '-' to clear the field or Press ENTER to use the current value").dim()
        );

        let cipher = Cipher::new(&master_pwd.to_string(), &self.key);

        let old_val = Value::decrypt(&cipher, &encrypted)?;

        let username = Prompt::username_with_default(&theme, old_val.username)?;
        let website = Prompt::website_with_default(&theme, old_val.website)?;
        let expires = Prompt::expires_with_default(&theme, old_val.expires)?;
        let notes = Prompt::notes_with_default(&theme, old_val.notes)?;

        let password = if self.password {
            let p = cipher.encrypt(&Password::generate(&self.pwd_opt).to_string())?;
            println!("{} Password regenerated", style(PREFIX).bold());
            p
        } else {
            cipher.decrypt(&old_val.password)?
        };

        let new_val = Value {
            password,
            username: username.unwrap_or_default(),
            website: website.unwrap_or_default(),
            expires: expires.unwrap_or_default(),
            notes: notes.unwrap_or_default(),
        }
        .encrypt(&cipher)?;

        let mut wtxn = db.write_txn()?;

        db.set(&mut wtxn, &self.key, &new_val.to_string())?;

        wtxn.commit()?;

        db.close();

        echo!("> Entry edited: {}", style(&self.key).bold());

        Ok(())
    }
}
