mod cli;
mod lib;
mod subcommand;

fn main() {
    let app = cli::parse();

    app.cmd.exec()

    // let code = match app.cmd.exec() {
    //     Ok(_) => 0,
    //     Err(e) => {
    //         eprintln!("ERROR :: {}", e.to_string().bright_red());
    //         1
    //     }
    // };
    //
    // std::process::exit(code)
}
