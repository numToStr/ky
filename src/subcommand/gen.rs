use crate::{
    cli::{Config, PwdGenOpts},
    lib::{KyError, Password},
    subcommand::Command,
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Generate {
    #[clap(flatten)]
    pwd_opt: PwdGenOpts,
}

impl Command for Generate {
    fn exec(&self, _: Config) -> Result<(), KyError> {
        let password = Password::generate(&self.pwd_opt);

        // Printing first part to stderr so that password can be easily read from stdout
        eprint!("Password: ");
        println!("{}", password);

        Ok(())
    }
}
