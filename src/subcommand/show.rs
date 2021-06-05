use super::Command;
use crate::{
    cli::Config,
    lib::{Cipher, Database, KyError, Password, Prompt, Value, MASTER},
};
use clap::Clap;
use tabled::{table, Alignment, Disable, Format, HorizontalAlignment, Row, Style, Tabled};

#[macro_export]
macro_rules! check_decrypt {
    ($cipher: expr, $encypted: expr) => {{
        use crate::lib::EMPTY;

        if $encypted != EMPTY {
            $cipher.decrypt($encypted)?
        } else {
            EMPTY.to_string()
        }
    }};
}

#[derive(Tabled)]
struct Detail(&'static str, String);

#[derive(Debug, Clap)]
pub struct Show {
    /// Unique key for the entry
    key: String,
}

impl Command for Show {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let master_pwd = Password::ask_master(&Prompt::theme())?;

        let db = Database::new(config.db_path())?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        // The crypted data returned from database
        // Will be in this format password:username:url:expires:notes
        let crypted = db.get(&self.key)?;
        let value = Value::from(crypted.as_str());

        let cipher = Cipher::new(&master_pwd.to_string());

        // We can use threads to decrypt each of them
        // and later use .join() to grab the decrypted value
        // Which will make this decryption way faster
        // I tried and I failed, maybe next time

        let decrypted = [
            Detail("Username", check_decrypt!(cipher, &value.keys.username)),
            Detail("Password", check_decrypt!(cipher, &value.keys.password)),
            Detail("URL", check_decrypt!(cipher, &value.keys.url)),
            Detail("Expires", check_decrypt!(cipher, &value.keys.expires)),
            Detail("Notes", check_decrypt!(cipher, &value.keys.notes)),
        ];

        let table = table!(
            &decrypted,
            Disable::Row(..1),
            Style::pseudo_clean().header(None),
            HorizontalAlignment(Row(..), Alignment::Left),
            Format(Row(..), |s| format!(" {} ", s))
        );

        // Don't println! because last line of table already contains a line feed
        print!("{}", table);

        Ok(())
    }
}
