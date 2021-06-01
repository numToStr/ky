use crate::{cli::Config, lib::KyError};
use clap::Clap;

use super::Command;

#[derive(Debug, Clap)]
pub struct Remove {}

impl Command for Remove {
    fn exec(&self, _: Config) -> Result<(), KyError> {
        Ok(())
    }
}
