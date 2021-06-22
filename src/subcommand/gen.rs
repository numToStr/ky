use crate::{
    cli::{Config, PasswordParams},
    lib::{KyResult, Password, Qr},
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
    fn exec(&self, _: Config) -> KyResult<()> {
        let password = Password::generate(&self.pwd_opt);
        let p = password.as_ref();

        if self.qr_code {
            let code = Qr::new(&p)?.render();
            eprint!("{}", code);
        }

        if !self.mute {
            // Printing first part to stderr so that password can be easily read from stdout
            eprint!("Password: ");
            println!("{}", p);
        }

        Ok(())
    }
}
