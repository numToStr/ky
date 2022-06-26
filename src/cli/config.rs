use clap::{crate_name, Parser};
use dirs::home_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
pub struct Config {
    /// Path to the vault directory
    #[clap(long, value_name = "path", env = "KY_VAULT_DIR")]
    vault_path: Option<PathBuf>,

    /// Path for the vault backup/export directory
    #[clap(long, value_name = "path", env = "KY_BACKUP_DIR")]
    backup_path: Option<PathBuf>,

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

const DB_NAME: &str = concat!(crate_name!(), ".db");
const BACKUP_NAME: &str = concat!(crate_name!(), ".backup");
const EXPORT_NAME: &str = concat!(crate_name!(), ".csv");

impl Config {
    pub fn ensure_create<P: AsRef<Path>>(&self, path: P) -> P {
        create_dir_all(&path).ok();
        path
    }

    pub fn ky_home(&self) -> PathBuf {
        home_dir()
            .expect("Unable to get the home directory")
            .join(concat!(".", crate_name!()))
    }

    pub fn backup_dir(&self) -> PathBuf {
        self.backup_path
            .clone()
            .unwrap_or_else(|| self.ky_home().join("backup"))
    }

    pub fn backup_path(&self) -> PathBuf {
        self.ensure_create(self.backup_dir()).join(BACKUP_NAME)
    }

    pub fn export_path(&self) -> PathBuf {
        self.ensure_create(self.backup_dir()).join(EXPORT_NAME)
    }

    pub fn vault_path(&self) -> PathBuf {
        self.vault_path
            .clone()
            .unwrap_or_else(|| self.ky_home().join("vault"))
    }

    pub fn db_path(&self) -> PathBuf {
        self.ensure_create(self.vault_path()).join(DB_NAME)
    }
}

/// Options for the auto generated password
#[derive(Debug, Parser)]
pub struct PasswordParams {
    /// Length of the generated password
    #[clap(short, long, default_value = "20")]
    pub length: u64,

    /// Characters to exclude from the password
    #[clap(short, long)]
    pub exclude: Option<String>,

    /// Characters set for the password
    #[clap(short, long)]
    pub charset: Option<String>,
}
