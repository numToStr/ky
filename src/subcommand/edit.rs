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
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Master::ask(&theme)?;

        let env = KyEnv::connect(&db_path)?;

        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;

        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        if !master_pwd.verify(hashed.as_ref())? {
            return Err(KyError::MisMatch);
        }

        let master_cipher = Cipher::for_master(&master_pwd);
        let enc_key = master_cipher.encrypt(&Decrypted::from(&self.key))?;

        let encrypted = pwd_db.get(&rtxn, &enc_key)?;

        rtxn.commit()?;

        println!(
            "  {}",
            style("Type '-' to clear the field or Press ENTER to use the current value").dim()
        );

        let key_cipher = Cipher::for_key(&master_pwd, &self.key)?;

        let old_val = Password::decrypt(&key_cipher, &encrypted)?;

        let username = Prompt::username_with_default(&theme, old_val.username)?;
        let website = Prompt::website_with_default(&theme, old_val.website)?;
        let expires = Prompt::expires_with_default(&theme, old_val.expires)?;
        let notes = Prompt::notes_with_default(&theme, old_val.notes)?;

        let password = if self.password {
            let p = Password::generate(&self.pwd_opt);
            println!("{} Password regenerated", style(PREFIX).bold());
            p
        } else {
            old_val.password
        };

        let new_val = Password {
            password,
            username,
            website,
            expires,
            notes,
        }
        .encrypt(&key_cipher)?;

        let mut wtxn = env.write_txn()?;

        pwd_db.set(&mut wtxn, &enc_key, &new_val)?;

        wtxn.commit()?;

        env.close();

        echo!("> Entry edited: {}", style(&self.key.as_ref()).bold());

        Ok(())
    }
}
