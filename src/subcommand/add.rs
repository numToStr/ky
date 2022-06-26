use std::convert::TryFrom;

use super::Command;
use crate::{
    cli::{Config, PasswordParams},
    echo,
    lib::{
        entity::{Master, Password},
        Cipher, EntryKey, KyDb2, KyError, KyResult, KyTable, Prompt,
    },
};
use clap::Parser;
use dialoguer::console::style;

#[derive(Debug, Parser)]
pub struct Add {
    /// Unique key for the entry
    key: EntryKey,

    /// Print newly created passoword
    #[clap(short, long)]
    print: bool,

    #[clap(flatten)]
    pwd_opt: PasswordParams,
}

impl Command for Add {
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

            if pwd_tbl.get(&key).is_ok() {
                return Err(KyError::Exist);
            }

            let username = Prompt::username(&theme)?;
            let website = Prompt::website(&theme)?;
            let expires = Prompt::expires(&theme)?;
            let notes = Prompt::note(&theme)?;

            let data_cipher = Cipher::try_from((&master, &self.key))?;

            let pwd = Password::generate(&self.pwd_opt);

            let encrypted = {
                let data = Password {
                    password: pwd.to_owned(),
                    username,
                    website,
                    expires,
                    note: notes,
                };
                data_cipher.encrypt(&data.into())?
            };

            pwd_tbl.set(key, encrypted)?;

            echo!("> Entry added: {}", style(&self.key.as_ref()).bold());

            if self.print {
                println!("> Password: {pwd}");
            }
        }

        wtxn.commit()?;

        Ok(())
    }
}
