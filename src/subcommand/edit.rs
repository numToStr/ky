use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{
        Cipher, Details, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable, Password, Prompt,
        MASTER, PREFIX,
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
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let env = KyEnv::connect(&db_path)?;

        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        let key_cipher = Cipher::for_key(&master_pwd);
        let key = key_cipher.encrypt(&self.key.as_ref())?;

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
            let p = Password::generate(&self.pwd_opt);
            println!("{} Password regenerated", style(PREFIX).bold());
            p.as_ref().to_string()
        } else {
            let p = cipher.decrypt(&old_val.password)?;
            p.as_ref().to_string()
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
