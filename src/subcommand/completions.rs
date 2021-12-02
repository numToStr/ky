use super::Command;
use crate::{
    cli::{Cli, Config},
    lib::{shell::Shell, KyResult},
};
use clap::{crate_name, IntoApp, Parser};
use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};

#[derive(Debug, Parser)]
pub struct Completions {
    #[clap(subcommand)]
    shell: Shell,
}

impl Command for Completions {
    fn exec(&self, _: Config) -> KyResult<()> {
        let name = crate_name!();
        let mut app = Cli::into_app();
        let mut fd = std::io::stdout();

        match self.shell {
            Shell::Bash => {
                generate(Bash, &mut app, name, &mut fd);
            }
            Shell::Zsh => {
                generate(Zsh, &mut app, name, &mut fd);
            }
            Shell::Fish => {
                generate(Fish, &mut app, name, &mut fd);
            }
            Shell::PowerShell => {
                generate(PowerShell, &mut app, name, &mut fd);
            }
            Shell::Elvish => {
                generate(Elvish, &mut app, name, &mut fd);
            }
        };

        Ok(())
    }
}
