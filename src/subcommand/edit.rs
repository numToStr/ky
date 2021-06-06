use super::Command;
use crate::{
    check_decrypt, check_encrypt,
    cli::{Config, PasswordParams},
    lib::{Cipher, Database, Keys, KyError, Password, Prompt, Value, MASTER, PREFIX},
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Edit {
    /// Entry which needs to be edited
    key: String,

    // /// Allow key of the entry to be edited
    // #[clap(short, long)]
    // key_edit: bool,
    /// Allow password to be regenerated
    #[clap(short, long)]
    password_gen: bool,

    #[clap(flatten)]
    pwd_opt: PasswordParams,
}

impl Command for Edit {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let db = Database::new(&config.db_path())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let encrypted = db.get(&self.key)?;

        println!();
        println!(
            "  {}",
            style("Type '-' to clear the field or Press ENTER to use the current value").dim()
        );

        let cipher = Cipher::new(&master_pwd.to_string());
        let value = Value::from(encrypted.as_str());

        let username_decrypted = check_decrypt!(cipher, &value.keys.username);
        let username = Prompt::username_with_default(&theme, username_decrypted)?;

        let url_decrypted = check_decrypt!(cipher, &value.keys.url);
        let url = Prompt::url_with_default(&theme, url_decrypted)?;

        let expires_decrypted = check_decrypt!(cipher, &value.keys.expires);
        let expires = Prompt::expires_with_default(&theme, expires_decrypted)?;

        let notes_decrypted = check_decrypt!(cipher, &value.keys.notes);
        let notes = Prompt::notes_with_default(&theme, notes_decrypted)?;

        let password = if self.password_gen {
            let p = cipher.encrypt(&Password::generate(&self.pwd_opt).to_string())?;
            println!("{} Password regenerated", style(PREFIX).bold());
            p
        } else {
            value.keys.password
        };

        let new_value = Value::new(Keys {
            password,
            username: check_encrypt!(cipher, username),
            url: check_encrypt!(cipher, url),
            expires: check_encrypt!(cipher, expires),
            notes: check_encrypt!(cipher, notes),
        });

        db.set(&self.key, &new_value.to_string())?;

        Ok(())
    }
}
