mod add;
mod backup;
mod edit;
mod export;
mod gen;
mod git;
mod init;
mod ls;
mod r#move;
mod nuke;
mod remove;
mod restore;
mod show;

use crate::{cli::Config, lib::KyError};
use clap::Subcommand;

use self::{
    add::Add, backup::Backup, edit::Edit, export::Export, gen::Generate, git::GitCmd, init::Init,
    ls::Ls, nuke::Nuke, r#move::Move, remove::Remove, restore::Restore, show::Show,
};

pub(self) trait Command {
    fn exec(&self, config: Config) -> Result<(), KyError>;
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Initialize the vault with a master password
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

    /// Backup the vault
    Backup(Backup),

    /// Restore the vault backup
    Restore(Restore),

    /// Permanently delete the local vault
    ///
    /// CAUTION: Please backup before doing this, otherwise you will loose all of your data.
    Nuke(Nuke),

    /// Use git to manage the vault
    Git(GitCmd),

    /// Rename the key and re-encrypt the entry's details
    #[clap(visible_alias = "mv")]
    Move(Move),

    /// Export vault in decrypted form
    ///
    /// CAUTION:
    /// Exported data files are not encrypted. They are stored in a plain text.
    /// Anyone with the access to those files will be able to read your password.
    Export(Export),
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
            Self::Backup(c) => c.exec(config),
            Self::Restore(c) => c.exec(config),
            Self::Nuke(c) => c.exec(config),
            Self::Git(c) => c.exec(config),
            Self::Move(c) => c.exec(config),
            Self::Export(c) => c.exec(config),
        }
    }
}

#[macro_export]
macro_rules! echo {
    ($($arg:tt)*) => {
        println!();
        println!($($arg)*);
    };
}
