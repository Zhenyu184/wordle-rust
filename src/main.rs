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
            let id: i64 = id_str.parse().expect("Invalid ID: must be a valid i64 integer");
            graph::show(id);
        },
        _ => {},
    }
}
