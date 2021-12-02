use super::Command;
use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{
        entity::Master, Cipher, Decrypted, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable,
        Prompt, MASTER,
    },
};
use clap::Parser;
use dialoguer::console::style;

#[derive(Debug, Parser)]
pub struct Remove {
    /// Entry which needs to be deleted
    key: EntryKey,
}

impl Command for Remove {
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

        let _ = pwd_db.get(&rtxn, &key)?;

        rtxn.commit()?;

        if Prompt::proceed(&theme)? {
            let mut wtxn = env.write_txn()?;

            pwd_db.delete(&mut wtxn, &key)?;

            echo!("> Entry deleted: {}", style(&self.key.as_ref()).bold());

            wtxn.commit()?;
        }

        env.close();

        Ok(())
    }
}
