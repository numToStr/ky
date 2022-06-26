mod add;
mod backup;
mod completions;
mod edit;
mod export;
mod gen;
mod git;
mod import;
mod init;
mod ls;
mod r#move;
mod nuke;
mod remove;
mod restore;
mod show;

use crate::{cli::Config, lib::KyResult};
use clap::Subcommand;

use self::{
    add::Add, backup::Backup, completions::Completion, edit::Edit, export::Export, gen::Generate,
    git::GitCmd, import::Import, init::Init, ls::Ls, nuke::Nuke, r#move::Move, remove::Remove,
    restore::Restore, show::Show,
};

pub(self) trait Command {
    fn exec(self, config: Config) -> KyResult<()>;
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Generate completions for different shells
    Completion(Completion),

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

    /// Export data as a csv file containing decrypted data
    ///
    /// CAUTION:
    /// Exported data files are not encrypted. They are stored in a plain text.
    /// Anyone with the access to those files will be able to read your password.
    Export(Export),

    /// Import data from a csv file containing decrypted data
    ///
    /// NOTE:
    /// Data you are importing should be in a clear text format.
    /// During import, you will be ask for a master password.
    /// Which will be used to encrypt all of your imported data.
    Import(Import),
}

impl SubCommand {
    pub fn exec(self, config: Config) -> KyResult<()> {
        match self {
            Self::Completion(c) => c.exec(config),
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
            Self::Import(c) => c.exec(config),
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
