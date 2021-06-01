use crate::{cli::Config, lib::KyError};
use clap::Clap;

use super::Command;

#[derive(Debug, Clap)]
pub struct Init {}

impl Command for Init {
    fn exec(&self, _: Config) -> Result<(), KyError> {
        Ok(())
    }
}
