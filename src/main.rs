mod game;

use clap::{App, AppSettings};

fn check_word() {
    println!("Check command!");
}

fn main() {
    let matches = App::new("wordle_rust")
        .version("0.1")
        .about("Wordle game")
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(App::new("start").about("Starts the game"))
        .subcommand(App::new("check").about("Check the word"))
        .get_matches();

    match matches.subcommand_name() {
        Some("start") => game::start_game(),
        Some("check") => check_word(),
        _ => {},
    }
}
