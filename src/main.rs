mod cli;
mod database;
mod game;

fn show() {
    println!("Check command!");
}

fn main() {
    let matches = cli::build_cli().get_matches();

    database::Database::client().lock().unwrap().init().expect("Failed to initialize database");

    match matches.subcommand_name() {
        Some("new") => game::new(),
        Some("show") => show(),
        _ => {},
    }
}
