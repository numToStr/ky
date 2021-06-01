use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Initialize the vault
    Init,

    /// Add a password in the vault
    Add,

    /// Remove a password from the vault
    Remove,
}

impl SubCommand {
    pub fn exec(&self) {
        match self {
            Self::Init => println!("Init"),
            Self::Add => println!("Add"),
            Self::Remove => println!("Remove"),
        }
    }
}
