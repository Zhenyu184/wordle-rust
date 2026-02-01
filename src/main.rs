mod cli;
mod game;
mod graph;
mod database;

fn main() {
    
    database::Database::connect().init().expect("Failed to initialize database");
    
    let matches = cli::build_cli().get_matches();
    
    match matches.subcommand() {
        ("new",   _) => game::new(),
        ("list",  _) => game::list(),
        ("clean", _) => game::clean(),
        ("show", Some(arg_matches)) => {
            let id_str = arg_matches.value_of("id").unwrap();
            let id: i64 = id_str.parse().expect("Invalid ID: must be a integer");
            graph::show(id);
        },
        ("submit", Some(arg_matches)) => {
            let id_str = arg_matches.value_of("id").unwrap();
            let id: i64 = id_str.parse().expect("Invalid ID: must be a integer");
            let word = arg_matches.value_of("word").unwrap();
            if word.len() != 5 || !word.chars().all(|c| c.is_ascii_alphabetic()) {
                println!("Word must be 5 English letters");
                return;
            }
            game::submit(id, word);
        },
        _ => {},
    }
}
