mod answer;

use crate::database;

pub fn new() {
    let answer = answer::generate_answer();

    let db = database::Database::connect();
    let id = db.add_game(56, &answer).expect("Failed to add game");

    println!("ID: {}", id);
}

pub fn list() {
    let db = database::Database::connect();
    let games = db.get_games().expect("Failed to list games");

    if games.is_empty() {
        println!("List is empty");
        return;
    }

    println!("{:<5} {:<10} {:<10} {:<15}", "ID", "Type", "Status", "Time");
    for (id, time, game_type, is_over) in games {
        let status = if is_over { "over" } else { "active" };
        println!("{:<5} {:<10} {:<10} {:<15}", id, game_type, status, time);
    }
}

pub fn clean() {
    let db = database::Database::connect();
    db.delete_all().expect("Failed to clean database");
    
    println!("All data cleaned");
}

pub fn submit(id: i64, word: &str) {
    let word = word.to_lowercase();
    let db = database::Database::connect();
    
    let Ok((_, _, _, is_over, answer, guesses)) = db.get_status(id) else {
        println!("Game ID {} not found", id);
        return;
    };

    if is_over {
        println!("Game is already over");
        return;
    }

    db.append_guesses(id, &word).expect("Failed to append guesses");

    let guess_count = guesses.as_deref().unwrap_or("").split(',').filter(|s| !s.is_empty()).count() + 1;

    if word != answer && guess_count < 6 {
        return;
    }

    db.set_game_over(id).expect("Failed to set game over");

    if word == answer {
        println!("🥳 Congratulations!");
    } else {
        println!("😵 GG");
    }
}