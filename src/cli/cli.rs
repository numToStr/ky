use super::Config;
use crate::subcommand::SubCommand;
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::DisableVersionForSubcommands
)]
pub struct Cli {
    #[clap(flatten)]
    pub config: Config,

    #[clap(subcommand)]
    pub cmd: SubCommand,
}

pub fn parse() -> Cli {
    Cli::parse()
}
