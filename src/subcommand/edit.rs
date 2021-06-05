use super::Command;
use crate::{
    cli::Config,
    lib::{Database, KyError, Password, Prompt, MASTER},
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Edit {
    /// Key which needs to be edited
    key: String,

    /// Allow key to be edited
    #[clap(short, long)]
    key_edit: bool,

    /// Allow password to be regenerated
    #[clap(short, long)]
    password_gen: bool,
}

impl Command for Edit {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db = Database::new(config.db_path())?;

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        // let decrypted = db.get(&self.key)?;

        let username = Prompt::username_with_default(&theme, "username".to_string())?;
        let url = Prompt::url_with_default(&theme, "url".to_string())?;
        let expires = Prompt::expires_with_default(&theme, "expires".to_string())?;
        let notes = Prompt::notes_with_default(&theme, "notes".to_string())?;

        println!();
        println!("{}", self.key);
        // println!("{}", decrypted);
        println!("{:#?}", username);
        println!("{:#?}", url);
        println!("{:#?}", expires);
        println!("{:#?}", notes);

        Ok(())
    }
}

// Enter master password:
// Type '-' to clear the field (except Name and Password) or leave blank to use the current value
// Name [gmail]: -
// Username [hello]: -
// Password:
// URL [fjfj]:
// Expires [Never]:
// Notes [] (type < to finish): <
