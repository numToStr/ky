use crate::{
    cli::{Config, PasswordParams},
    lib::{entity::Password, KyResult, Qr},
    subcommand::Command,
};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Generate {
    #[clap(flatten)]
    pwd_opt: PasswordParams,

    /// Show password in a form of qr code
    #[clap(short, long)]
    qr_code: bool,

    /// Don't print the password, can be used with qr code
    #[clap(short, long)]
    mute: bool,
}

impl Command for Generate {
    fn exec(self, _: Config) -> KyResult<()> {
        let password = Password::generate(&self.pwd_opt);

        if self.qr_code {
            let code = Qr::new(&password)?.render();
            eprint!("{}", code);
        }

        if !self.mute {
            println!("{}", password);
        }

        Ok(())
    }
}
