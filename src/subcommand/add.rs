use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{
        entity::Master, Cipher, Decrypted, Details, Encrypted, EntryKey, KyEnv, KyError, KyResult,
        KyTable, Password, Prompt, MASTER,
    },
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Add {
    /// Unique key for the entry
    key: EntryKey,

    #[clap(flatten)]
    pwd_opt: PasswordParams,
}

impl Command for Add {
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

        if pwd_db.get(&rtxn, &key).is_ok() {
            return Err(KyError::Exist);
        }

        rtxn.commit()?;

        let username = Prompt::username(&theme)?;
        let website = Prompt::website(&theme)?;
        let expires = Prompt::expires(&theme)?;
        let notes = Prompt::notes(&theme)?;

        let cipher = Cipher::for_value(&master, &self.key)?;

        let new_pass = Password::generate(&self.pwd_opt);

        let encrypted = Details {
            password: new_pass.as_ref().to_string(),
            username,
            website,
            expires,
            notes,
        }
        .encrypt(&cipher)?;

        let mut wtxn = env.write_txn()?;

        pwd_db.set(&mut wtxn, &key, &encrypted)?;

        wtxn.commit()?;

        env.close();

        echo!("> Entry added: {}", style(&self.key.as_ref()).bold());

        Ok(())
    }
}
