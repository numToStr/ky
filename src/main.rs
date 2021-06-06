use dialoguer::console::style;

mod cli;
mod lib;
mod subcommand;

fn main() {
    let app = cli::parse();

    let code = match app.cmd.exec(app.config) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{} :: {}", style("ERROR").red().bright(), e.to_string());
            1
        }
    };

    std::process::exit(code)
}
