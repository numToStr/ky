use super::Command;
use crate::{
    cli::{Cli, Config},
    lib::KyResult,
};
use clap::{crate_name, IntoApp, Parser};
use clap_complete::{generate, Shell};

#[derive(Debug, Parser)]
pub struct Completion {
    /// Name of the shell
    #[clap(arg_enum)]
    shell: Shell,
}

impl Command for Completion {
    fn exec(&self, _: Config) -> KyResult<()> {
        generate(
            self.shell,
            &mut Cli::into_app(),
            crate_name!(),
            &mut std::io::stdout(),
        );

        Ok(())
    }
}
