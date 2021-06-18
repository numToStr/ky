use super::Command;
use crate::{
    cli::{Cli, Config},
    lib::{shell::Shell, KyError},
};
use clap::{crate_name, Clap, IntoApp};
use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};

#[derive(Debug, Clap)]
pub struct Completions {
    #[clap(subcommand)]
    shell: Shell,
}

impl Command for Completions {
    fn exec(&self, _: Config) -> Result<(), KyError> {
        let name = crate_name!();
        let mut app = Cli::into_app();
        let mut fd = std::io::stdout();

        match self.shell {
            Shell::Bash => {
                generate::<Bash, _>(&mut app, name, &mut fd);
            }
            Shell::Zsh => {
                generate::<Zsh, _>(&mut app, name, &mut fd);
            }
            Shell::Fish => {
                generate::<Fish, _>(&mut app, name, &mut fd);
            }
            Shell::PowerShell => {
                generate::<PowerShell, _>(&mut app, name, &mut fd);
            }
            Shell::Elvish => {
                generate::<Elvish, _>(&mut app, name, &mut fd);
            }
        };

        Ok(())
    }
}