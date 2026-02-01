mod answer;

use crate::database;

pub fn new() {
    let answer = answer::generate_answer();
    let id = database::Database::connect().add_game(0, &answer).expect("Failed to add game");

    println!("Answer: {}", answer);
    println!("Game started! (id: {})", id);
}

pub fn list() {
    let games = database::Database::connect().get_games().expect("Failed to list games");

    println!("{:<5} {:<10} {:<10} {:<15}", "ID", "Type", "Status", "Time");
    for (id, time, game_type, is_over) in games {
        let status = if is_over { "Done" } else { "Active" };
        println!("{:<5} {:<10} {:<10} {:<15}", id, game_type, status, time);
    }
}

pub fn clean() {
    database::Database::connect().delete_all().expect("Failed to clean database");
    println!("All data cleaned");
}