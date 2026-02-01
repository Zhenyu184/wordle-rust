mod cli;
mod game;

fn check_word() {
    println!("Check command!");
}

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand_name() {
        Some("start") => game::start_game(),
        Some("check") => check_word(),
        _ => {},
    }
}
