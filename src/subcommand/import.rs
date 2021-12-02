use crate::{
    cli::Config,
    echo,
    lib::{entity::Master, KyEnv, KyError, KyResult, Prompt, Vault},
};
use clap::Parser;

use std::{fs::remove_dir_all, path::PathBuf};

use super::Command;

#[derive(Debug, Parser)]
pub struct Import {
    /// Path to the exported file
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// Ignore already initialized vault, if any
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for Import {
    fn exec(&self, config: Config) -> KyResult<()> {
        let import_path = match &self.path {
            Some(p) => p.to_path_buf(),
            _ => config.export_path(),
        };

        if !import_path.exists() {
            return Err(KyError::RestoreDontExist);
        }

        let theme = Prompt::theme();

        let db_path = config.db_path();
        let db_exist = db_path.exists();

        if !self.ignore && db_exist && !Prompt::vault_exist(&theme)? {
            return Ok(());
        }

        let master = Master::new(&theme)?;

        if db_exist {
            remove_dir_all(&db_path)?;
        }

        let env = KyEnv::connect(config.ensure_create(&db_path))?;

        Vault::import(&import_path, &master, &env)?;

        env.close();

        echo!("> Vault imported!");

        Ok(())
    }
}
