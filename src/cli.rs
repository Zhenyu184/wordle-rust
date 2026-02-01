use clap::{App, Arg, AppSettings};

fn arg_id() -> Arg<'static, 'static> {
    Arg::with_name("id")
        .help("The game ID")
        .long("id")
        .short("i")
        .required(true)
        .takes_value(true)
}

fn arg_word() -> Arg<'static, 'static> {
    Arg::with_name("word")
        .help("The word to guess")
        .required(true)
        .index(1)
}

pub fn build_cli() -> App<'static, 'static> {
    App::new("wordle_rust")
        .version("0.1")
        .about("Wordle game")
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            App::new("new").about("Starts new game")
        )
        .subcommand(
            App::new("list").about("List games")
        )
        .subcommand(
            App::new("show")
                .about("Show game current status")
                .arg(arg_id()),
        )
        .subcommand(
            App::new("clean").about("clean all games")
        )
        .subcommand(
            App::new("submit").about("summit the answer")
                .arg(arg_id())
                .arg(arg_word()),
        )
}