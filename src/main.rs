mod cli;
mod game;

fn show() {
    println!("Check command!");
}

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand_name() {
        Some("new") => game::new(),
        Some("show") => show(),
        _ => {},
    }
}
