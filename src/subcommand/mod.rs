mod add;
mod gen;
mod init;
mod remove;
mod show;

use crate::{cli::Config, lib::KyError};
use clap::Subcommand;

use self::{add::Add, gen::Generate, init::Init, remove::Remove, show::Show};

pub(self) const MASTER: &str = "master";

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

    /// Show the password
    Show(Show),

    /// Generate random and cryptographically strong password
    Gen(Generate),
}

impl SubCommand {
    pub fn exec(&self, config: Config) -> Result<(), KyError> {
        match self {
            Self::Init(c) => c.exec(config),
            Self::Add(c) => c.exec(config),
            Self::Remove(c) => c.exec(config),
            Self::Show(c) => c.exec(config),
            Self::Gen(c) => c.exec(config),
        }
    }
}
