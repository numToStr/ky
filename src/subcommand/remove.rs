use super::Command;
use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{Database2, KyError, Password, Prompt, MASTER},
};
use clap::Clap;
use dialoguer::console::style;

#[derive(Debug, Clap)]
pub struct Remove {
    /// Entry which needs to be deleted
    key: String,
}

impl Command for Remove {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let env = Database2::env(&db_path)?;
        let txn = env.begin_rw_txn()?;

        let db = Database2::open(&txn)?;

        let hashed = db.get(MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        let res = match db.get(&self.key) {
            Ok(val) => {
                if Prompt::proceed(&theme)? {
                    db.delete(&self.key, &val)?;

                    echo!("> Entry deleted: {}", style(&self.key).bold());
                }

                Ok(())
            }
            _ => Err(KyError::NotFound(self.key.to_string())),
        };

        txn.commit()?;

        res
    }
}
