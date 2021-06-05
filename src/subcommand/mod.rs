mod add;
mod gen;
mod init;
mod ls;
mod remove;
mod show;

use crate::{cli::Config, lib::KyError};
use clap::Subcommand;

use self::{add::Add, gen::Generate, init::Init, ls::Ls, remove::Remove, show::Show};

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

    /// Print a tree view of all keys present in the vault
    Ls(Ls),
}

impl SubCommand {
    pub fn exec(&self, config: Config) -> Result<(), KyError> {
        match self {
            Self::Init(c) => c.exec(config),
            Self::Add(c) => c.exec(config),
            Self::Remove(c) => c.exec(config),
            Self::Show(c) => c.exec(config),
            Self::Gen(c) => c.exec(config),
            Self::Ls(c) => c.exec(config),
        }
    }
}
