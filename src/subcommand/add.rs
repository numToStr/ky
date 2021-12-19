use super::Command;
use crate::{
    check_db,
    cli::{Config, PasswordParams},
    echo,
    lib::{
        entity::{Master, Password},
        Cipher, Decrypted, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable, Prompt, MASTER,
    },
};
use clap::Parser;
use dialoguer::console::style;

#[derive(Debug, Parser)]
pub struct Add {
    /// Unique key for the entry
    key: EntryKey,

    /// Print newly created passoword
    #[clap(short = 'P', long)]
    print: bool,

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

        let master_cipher = Cipher::for_master(&master);
        let enc_key = master_cipher.encrypt(&Decrypted::from(&self.key))?;

        if pwd_db.get(&rtxn, &enc_key).is_ok() {
            return Err(KyError::Exist);
        }

        rtxn.commit()?;

        let username = Prompt::username(&theme)?;
        let website = Prompt::website(&theme)?;
        let expires = Prompt::expires(&theme)?;
        let notes = Prompt::notes(&theme)?;

        let key_cipher = Cipher::for_key(&master, &self.key)?;

        let password = Password::generate(&self.pwd_opt);

        let encrypted = Password {
            password: password.to_string(),
            username,
            website,
            expires,
            notes,
        }
        .encrypt(&key_cipher)?;

        let mut wtxn = env.write_txn()?;

        pwd_db.set(&mut wtxn, &enc_key, &encrypted)?;

        wtxn.commit()?;

        env.close();

        echo!("> Entry added: {}", style(&self.key.as_ref()).bold());
        if self.print {
            println!("> Password: {}", password);
        }

        Ok(())
    }
}
