use crate::database;

pub fn show(id: i64) {
    let db = database::Database::connect();
    match db.get_game(id) {
        Ok((id, time, type_, is_over, answer, guesses)) => {
            println!("ID:      {}", id);
            println!("Type:    {}", type_);
            println!("Time:    {}", time);
            println!("Status:  {}", if is_over { "Finished" } else { "Active" });
            println!("Answer:  {}", answer);
            println!("Guesses: {}", guesses.unwrap_or_else(|| "None".to_string()));
        },
        Err(_) => println!("Game {} not found", id),
    }
}