use clap::crate_name;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KyError {
    // Generic and Unknown error, that I don't want to handle
    #[error("Something went wrong! {0}")]
    Any(String),

    // #startregion: Errors related to database
    #[error("Vault is already initialized!")]
    Init,

    #[error("Vault is not initialized. Please run `{} init` to initialize the vault!", crate_name!())]
    NoInit,

    #[error("Unable to establish database connection!")]
    Connection,

    #[error("Vault backup not found on the provided path!")]
    BackupDontExist,

    #[error("Entry not found in the vault: `{0}`")]
    NotFound(String),

    #[error("Entry already exist in the vault: `{0}`")]
    Exist(String),

    #[error("Unable to get the entry: `{0}`")]
    Get(String),

    #[error("Unable to set the entry: `{0}`")]
    Set(String),

    #[error("Unable to delete the entry: `{0}`")]
    Delete(String),
    // #endregion
    #[error("Unable to hash the password!")]
    Hashing,

    #[error("Password mismatch!")]
    MisMatch,

    #[error("Unable to decrypt the provided data!")]
    Decrypt,

    #[error("Unable to encrypt the provided data!")]
    Encrypt,

    #[error("Unable to spawn `git`. Make sure you have git installed!")]
    Git,

    #[error("Git is already initialized!")]
    GitInit,

    #[error("Git is not initialized. Please run `{} git init` to initialize a git repo!", crate_name!())]
    GitNoInit,

    #[error("Git repository is not set. Make sure have added `KY_GIT_REPO` environment variable!")]
    GitRepo,

    #[error(
        "Git default branch is not set. Make sure have added `KY_GIT_BRANCH` environment variable!"
    )]
    GitBranch,
}

impl From<io::Error> for KyError {
    fn from(s: io::Error) -> Self {
        Self::Any(s.to_string())
    }
}

// impl From<heed::Error> for KyError {
//     fn from(s: heed::Error) -> Self {
//         Self::Any(s.to_string())
//     }
// }

impl From<mdbx::Error> for KyError {
    fn from(s: mdbx::Error) -> Self {
        Self::Any(s.to_string())
    }
}
