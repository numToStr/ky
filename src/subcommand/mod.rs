mod add;
mod init;
mod remove;

use crate::{cli::Config, lib::KyError};
use clap::Subcommand;

use self::{add::Add, init::Init, remove::Remove};

pub(self) trait Command {
    fn exec(&self, config: Config) -> Result<(), KyError>;
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Initialize the vault
    Init(Init),

    /// Add a password in the vault
    Add(Add),

    /// Remove a password from the vault
    Remove(Remove),
}

impl SubCommand {
    pub fn exec(&self, config: Config) -> Result<(), KyError> {
        match self {
            Self::Init(c) => c.exec(config),
            Self::Add(c) => c.exec(config),
            Self::Remove(c) => c.exec(config),
        }
    }
}
