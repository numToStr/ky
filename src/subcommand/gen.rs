use crate::{
    cli::Config,
    lib::{KyError, Password},
    subcommand::Command,
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Generate {
    /// Length of the password
    #[clap(short, long, default_value = "20")]
    length: u64,
}

impl Command for Generate {
    fn exec(&self, _: Config) -> Result<(), KyError> {
        let password = Password::generate(self.length);

        println!("{}", password);

        Ok(())
    }
}
