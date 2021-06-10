use super::Command;
use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{Database, KyError, Password, Prompt, MASTER},
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

        let db = Database::open(&db_path)?;

        let rtxn = db.read_txn()?;
        let hashed = db.get(&rtxn, MASTER)?;

        if !master_pwd.verify(&hashed) {
            return Err(KyError::MisMatch);
        }

        rtxn.commit()?;

        if Prompt::proceed(&theme)? {
            let mut wtxn = db.write_txn()?;

            match db.delete(&mut wtxn, &self.key)? {
                true => {
                    echo!("> Entry deleted: {}", style(&self.key).bold());
                }
                _ => {
                    return Err(KyError::NotFound(self.key.to_string()));
                }
            };

            wtxn.commit()?;
        }

        Ok(())
    }
}
