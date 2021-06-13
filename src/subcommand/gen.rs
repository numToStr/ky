use crate::{
    cli::{Config, PasswordParams},
    lib::{KyError, Password, Qr},
    subcommand::Command,
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Generate {
    #[clap(flatten)]
    pwd_opt: PasswordParams,

    /// Show password in a form of qr code
    #[clap(short, long)]
    qr_code: bool,

    /// Don't print the details, can be used with qr code
    #[clap(short, long)]
    mute: bool,
}

impl Command for Generate {
    fn exec(&self, _: Config) -> Result<(), KyError> {
        let password = Password::generate(&self.pwd_opt);

        if self.qr_code {
            let code = Qr::new(&password.to_string()).render();
            eprint!("{}", code);
        }

        if !self.mute {
            // Printing first part to stderr so that password can be easily read from stdout
            eprint!("Password: ");
            println!("{}", password);
        }

        Ok(())
    }
}
