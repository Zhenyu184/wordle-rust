mod answer;
use crate::database;

pub fn new() {
    let answer = answer::generate_answer();
    let id = database::Database::connect().add_game(&answer).expect("Failed to add game");
    println!("Answer: {}", answer);
    println!("Game started! (id: {})", id);
}