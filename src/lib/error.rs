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

    #[error("Entry not found in the vault!")]
    NotFound,

    #[error("Entry already exist in the vault!")]
    Exist,

    #[error("Unable to get the entry!")]
    Get,

    #[error("Unable to set the entry!")]
    Set,

    #[error("Unable to delete the entry~")]
    Delete,
    // #endregion
    #[error("Unable to hash the password!")]
    PwdHash,

    #[error("Unable to verify the password. Make sure your password is properly hashed!")]
    PwdVerify,

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
    // QrCode
    #[error("Unable to generate qr-code. Make sure your data is valid!")]
    Qr,
    // Import/Export
    #[error("Unable to create the export file!")]
    ExportCreate,

    #[error("Unable to export data related to {0} key!")]
    Export(String),

    #[error("Unable to read the import file!")]
    ImportRead,

    #[error("Unable to import data for row {0}!")]
    Import(usize),

    #[error("Vault export not found on the provided path!")]
    RestoreDontExist,
}

impl From<io::Error> for KyError {
    fn from(s: io::Error) -> Self {
        Self::Any(s.to_string())
    }
}

impl From<heed::Error> for KyError {
    fn from(s: heed::Error) -> Self {
        Self::Any(s.to_string())
    }
}
