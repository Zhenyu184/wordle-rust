mod cli;
mod game;
mod database;

fn show() {
    println!("Check command!");
}

fn main() {
    let matches = cli::build_cli().get_matches();

    database::Database::connect().init().expect("Failed to initialize database");

    match matches.subcommand_name() {
        Some("new") => game::new(),
        Some("list") => game::list(),
        Some("clean") => game::clean(),
        Some("show") => show(),
        _ => {},
    }
}
