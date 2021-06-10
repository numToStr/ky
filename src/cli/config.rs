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
    prompt: String,

    /// Git repo used for backup storage (Hidden)
    #[clap(long, name = "repo", env = "KY_GIT_REPO", hide_env_values = true)]
    pub git_repo: Option<String>,

    /// Default branch for the git repo (Hidden)
    #[clap(long, name = "branch", env = "KY_GIT_BRANCH", hide_env_values = true)]
    pub git_branch: Option<String>,
}

impl Config {
    pub fn ensure_create<P: AsRef<Path>>(&self, path: P) -> P {
        create_dir_all(&path).ok();
        path
    }

    fn ky_home(&self) -> PathBuf {
        home_dir()
            .expect("Unable to get the home directory")
            .join(concat!(".", crate_name!()))
    }

    pub fn backup_path(&self) -> PathBuf {
        self.ensure_create(self.ky_home().join("backup"))
            .join(concat!(crate_name!(), ".backup"))
    }

    pub fn db_path(&self) -> PathBuf {
        self.ensure_create(
            self.vault_path
                .clone()
                .unwrap_or_else(|| self.ky_home().join("vault")),
        )
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
