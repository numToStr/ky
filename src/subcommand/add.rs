use crate::{cli::Config, lib::KyError};
use clap::Clap;

use super::Command;

#[derive(Debug, Clap)]
pub struct Add {
    /// Unique key for the entry
    key: String,

    /// Length of the password
    #[clap(short, long, default_value = "20")]
    length: u64,
}

impl Command for Add {
    fn exec(&self, _: Config) -> Result<(), KyError> {
        Ok(())
    }
}
