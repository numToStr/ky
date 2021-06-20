use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{
        key::EntryKey, Cipher, Details, KyEnv, KyError, KyTable, Password, Prompt, MASTER, PREFIX,
    },
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
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
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let env = KyEnv::connect(&db_path)?;

        let master_db = env.get_table(KyTable::Master)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = master_db.get(&rtxn, MASTER)?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        let key = Cipher::for_key(&master_pwd).encrypt(&self.key.as_ref())?;

        let encrypted = pwd_db.get(&rtxn, &key)?;

        rtxn.commit()?;

        echo!(
            "  {}",
            style("Type '-' to clear the field or Press ENTER to use the current value").dim()
        );

        let cipher = Cipher::for_value(&master_pwd, &self.key)?;

        let old_val = Details::decrypt(&cipher, &encrypted)?;

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

        let new_val = Details {
            password,
            username,
            website,
            expires,
            notes,
        }
        .encrypt(&cipher)?;

        let mut wtxn = env.write_txn()?;

        pwd_db.set(&mut wtxn, &key, &new_val)?;

        wtxn.commit()?;

        env.close();

        echo!("> Entry edited: {}", style(&self.key.as_ref()).bold());

        Ok(())
    }
}
