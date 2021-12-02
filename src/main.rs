mod cli;
mod lib;
mod subcommand;

use clap::Parser;
use cli::Cli;
use dialoguer::console::style;

fn main() {
    let app = Cli::parse();

    let code = match app.cmd.exec(app.config) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{} :: {}", style("ERROR").red().bright(), e.to_string());
            1
        }
    };

    std::process::exit(code)
}
