use clap::{Arg, Command};

fn arg_id() -> Arg {
    Arg::new("id")
        .help("The game ID")
        .long("id")
        .short('i')
        .required(true)
}

fn arg_word() -> Arg {
    Arg::new("word")
        .help("The word to guess")
        .required(true)
        .index(1)
}

pub fn build_cli() -> Command {
    Command::new("wordle_rust")
        .version("0.1")
        .about("Wordle game")
        .subcommand(
            Command::new("new").about("Starts new game")
        )
        .subcommand(
            Command::new("list").about("List games")
        )
        .subcommand(
            Command::new("show")
                .about("Show game current status")
                .arg(arg_id()),
        )
        .subcommand(
            Command::new("clean").about("clean all games")
        )
        .subcommand(
            Command::new("submit").about("summit the answer")
                .arg(arg_id())
                .arg(arg_word()),
        )
}