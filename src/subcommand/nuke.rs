use crate::{
    check_db,
    cli::Config,
    echo,
    lib::{Encrypted, KyEnv, KyError, KyResult, KyTable, Password, Prompt, MASTER},
};
use clap::Clap;
use std::fs::remove_dir_all;

use super::Command;

#[derive(Debug, Clap)]
pub struct Nuke {
    /// Delete everything, including default backup (if any)
    #[clap(short, long)]
    all: bool,
}

impl Command for Nuke {
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let theme = Prompt::theme();
        let master_pwd = Password::ask_master(&theme)?;

        let env = KyEnv::connect(&db_path)?;
        let common_db = env.get_table(KyTable::Common)?;

        let rtxn = env.read_txn()?;
        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;
        rtxn.commit()?;

        env.close();

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        let proceed = Prompt::proceed(&theme)?;

        match (proceed, self.all) {
            (true, true) => {
                remove_dir_all(config.ky_home())?;
                echo!("> Everything nuked!");
            }
            (true, false) => {
                remove_dir_all(db_path)?;
                echo!("> Vault nuked!");
            }
            _ => {}
        }

        Ok(())
    }
}
