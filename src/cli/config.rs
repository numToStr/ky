use clap::{crate_name, Clap};
use std::path::PathBuf;

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
    pub fn db_path(self) -> PathBuf {
        self.vault_path
            .unwrap_or_else(|| PathBuf::new().join("lok.db"))
    }
}

/// Options for the auto generated password
#[derive(Debug, Clone, Clap)]
pub struct PwdGenOpts {
    /// Length of the generated password
    #[clap(short, long, default_value = "20")]
    pub length: u64,
}
