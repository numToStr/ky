mod add;
mod edit;
mod gen;
mod init;
mod ls;
mod remove;
mod show;

use crate::{cli::Config, lib::KyError};
use clap::Subcommand;

use self::{add::Add, edit::Edit, gen::Generate, init::Init, ls::Ls, remove::Remove, show::Show};

pub(self) trait Command {
    fn exec(&self, config: Config) -> Result<(), KyError>;
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Initialize the vault
    #[clap(visible_alias = "i")]
    Init(Init),

    /// Add an entry to the vault
    #[clap(visible_alias = "a")]
    Add(Add),

    /// Remove an entry from the vault
    #[clap(visible_alias = "rm")]
    Remove(Remove),

    /// Show details of an entry
    Show(Show),

    /// Generate random and cryptographically strong password
    Gen(Generate),

    /// Print a tree view of all keys present in the vault
    #[clap(visible_alias = "ls")]
    List(Ls),

    /// Edit a existing entry present in the vault
    #[clap(visible_alias = "e")]
    Edit(Edit),
}

impl SubCommand {
    pub fn exec(&self, config: Config) -> Result<(), KyError> {
        match self {
            Self::Init(c) => c.exec(config),
            Self::Add(c) => c.exec(config),
            Self::Remove(c) => c.exec(config),
            Self::Show(c) => c.exec(config),
            Self::Gen(c) => c.exec(config),
            Self::List(c) => c.exec(config),
            Self::Edit(c) => c.exec(config),
        }
    }
}
