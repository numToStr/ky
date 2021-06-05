use clap::{crate_name, Clap};
use dirs::home_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Clap, Debug)]
pub struct Config {
    /// Path to the password vault
    #[clap(long, name = "path", env = "KY_VAULT")]
    vault_path: Option<PathBuf>,

    /// Prompt used inside the session
    #[clap(long, env = "KY_PROMPT", default_value = concat!(crate_name!(), " $"))]
    prompt: PathBuf,
}

impl Config {
    fn ensure_create<P: AsRef<Path>>(&self, path: P) -> P {
        create_dir_all(&path).ok();
        path
    }

    fn ky_home(&self) -> PathBuf {
        home_dir()
            .expect("Unable to get the home directory")
            .join(concat!(".", crate_name!()))
    }

    pub fn db_path(self) -> PathBuf {
        self.ensure_create(self.vault_path.clone().unwrap_or_else(|| self.ky_home()))
            .join(concat!(crate_name!(), ".db"))
    }
}

/// Options for the auto generated password
#[derive(Debug, Clone, Clap)]
pub struct PasswordParams {
    /// Length of the generated password
    #[clap(short, long, default_value = "20")]
    pub length: u64,

    /// Characters to exclude from the password
    #[clap(short, long, name = "chars")]
    pub exclude: Option<String>,
}
