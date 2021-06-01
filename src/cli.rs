use std::path::PathBuf;

use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};

use crate::subcommand::SubCommand;

#[derive(Clap, Debug)]
pub struct Config {
    #[clap(long, env = "KY_DB")]
    db_path: Option<PathBuf>,
}

impl Config {
    pub fn db_path(self) -> PathBuf {
        self.db_path
            .unwrap_or_else(|| PathBuf::new().join("lok.db"))
    }
}

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
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
