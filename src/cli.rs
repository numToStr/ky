use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};

use crate::subcommand::SubCommand;

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
)]
pub struct Cli {
    // #[clap(flatten)]
    // pub options: Config,
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

pub fn parse() -> Cli {
    Cli::parse()
}
