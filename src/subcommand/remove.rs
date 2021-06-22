use super::Command;
use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{
        Cipher, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable, Password, Prompt, MASTER,
    },
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Remove {
    /// Entry which needs to be deleted
    key: EntryKey,
}

impl Command for Remove {
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
