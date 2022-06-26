use std::convert::TryFrom;

use super::Command;
use crate::{
    cli::{Config, PasswordParams},
    echo,
    lib::{
        entity::{Master, Password},
        Cipher, EntryKey, KyDb2, KyError, KyResult, KyTable, Prompt, PREFIX,
    },
};
use clap::Parser;
use dialoguer::console::style;

#[derive(Debug, Parser)]
pub struct Edit {
    /// Entry which needs to be edited
    key: EntryKey,

    /// Allow password to be regenerated
    #[clap(short, long)]
    password: bool,

    #[clap(flatten)]
    pwd_opt: PasswordParams,
}

impl Command for Edit {
    fn exec(self, config: Config) -> KyResult<()> {
        let theme = Prompt::theme();
        let master = Master::ask(&theme)?;

        let db = KyDb2::connect(&config.db_path())?;

        {
            let rtxn = db.rtxn()?;
            let master_tbl = db.open_read(&rtxn, KyTable::Master)?;
            let hashed = master_tbl.get(&Master::KEY.into())?;

            if !master.verify(hashed)? {
                return Err(KyError::MisMatch);
            }
        }

        let wtxn = db.wtxn()?;

        {
            let mut pwd_tbl = db.open_write(&wtxn, KyTable::Password)?;
            let key = Cipher::from(&master).encrypt(&self.key.clone().into())?;
            let encrypted = pwd_tbl.get(&key)?;

            println!(
                "  {}",
                style("Type '-' to clear the field or Press ENTER to use the current value").dim()
            );

            let data_cipher = Cipher::try_from((&master, &self.key))?;

            let old_val = Password::try_from(data_cipher.decrypt(&encrypted)?)?;

            let username = Prompt::username_with_default(&theme, old_val.username)?;
            let website = Prompt::website_with_default(&theme, old_val.website)?;
            let expires = Prompt::expires_with_default(&theme, old_val.expires)?;
            let notes = Prompt::notes_with_default(&theme, old_val.note)?;

            let password = if self.password {
                let p = Password::generate(&self.pwd_opt);
                println!("{} Password regenerated", style(PREFIX).bold());
                p
            } else {
                old_val.password
            };

            let new_val = {
                let data = Password {
                    password,
                    username,
                    website,
                    expires,
                    note: notes,
                };
                data_cipher.encrypt(&data.into())?
            };

            pwd_tbl.set(key, new_val)?;
        }

        wtxn.commit()?;

        echo!("> Entry edited: {}", style(&self.key.as_ref()).bold());

        Ok(())
    }
}
