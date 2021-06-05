use crate::{
    cli::Config,
    lib::{Cipher, Database, KyError, Password, Prompt},
};
use clap::Clap;
use tabled::{
    table, Alignment, AlignmentObject, ChangeRing, HorizontalAlignment, Row, Style, Tabled,
};

use super::{Command, MASTER};

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
        let data = db.get(&self.key)?;
        let mut crypted = data.splitn(5, ':');

        let cipher = Cipher::new(&master_pwd.to_string());

        // We can use threads to decrypt each of them
        // and later use .join() to grab the decrypted value
        // Which will make this decryption way faster
        // I tried and I failed, maybe next time
        let password = crypted.next();
        let username = crypted.next();
        let url = crypted.next();
        let expires = crypted.next();
        let notes = crypted.next();

        let decrypted = [
            Detail("Username", check_decrypt(&cipher, username)?),
            Detail("Password", check_decrypt(&cipher, password)?),
            Detail("URL", check_decrypt(&cipher, url)?),
            Detail("Expires", check_decrypt(&cipher, expires)?),
            Detail("Notes", check_decrypt(&cipher, notes)?),
        ];

        let table = table!(
            &decrypted,
            Style::PseudoClean,
            HorizontalAlignment::new(Alignment::Left, AlignmentObject::Full),
            ChangeRing(Row(1..), vec![Box::new(|s| format!(" {} ", s))])
        );

        println!("{}", table);

        Ok(())
    }
}

#[inline]
fn check_decrypt(cipher: &Cipher, encypted: Option<&str>) -> Result<String, KyError> {
    match encypted {
        Some(x) if x != "-" => cipher.decrypt(encypted.unwrap()),
        _ => Ok("-".to_string()),
    }
}
