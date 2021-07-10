use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{
        entity::{Master, Password},
        Cipher, Decrypted, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable, Prompt, MASTER,
        PREFIX,
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
        let master = Master::ask(&theme)?;

        let env = KyEnv::connect(&db_path)?;

        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        if !master.verify(hashed.as_ref())? {
            return Err(KyError::MisMatch);
        }

        let key_cipher = Cipher::for_key(&master);
        let key = key_cipher.encrypt(&Decrypted::from(&self.key))?;

        let encrypted = pwd_db.get(&rtxn, &key)?;

        rtxn.commit()?;

        echo!(
            "  {}",
            style("Type '-' to clear the field or Press ENTER to use the current value").dim()
        );

        let cipher = Cipher::for_value(&master, &self.key)?;

        let old_val = Password::decrypt(&cipher, &encrypted)?;

        let username = Prompt::username_with_default(&theme, old_val.username)?;
        let website = Prompt::website_with_default(&theme, old_val.website)?;
        let expires = Prompt::expires_with_default(&theme, old_val.expires)?;
        let notes = Prompt::notes_with_default(&theme, old_val.notes)?;

        let password = if self.password {
            let p = Password::generate(&self.pwd_opt);
            println!("{} Password regenerated", style(PREFIX).bold());
            p
        } else {
            let p = cipher.decrypt(&Encrypted::from(old_val.password))?;
            p.as_ref().to_string()
        };

        let new_val = Password {
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
