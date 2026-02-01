use crate::database;



pub fn show(id: i64) {
    let db = database::Database::connect();
    let data = db.get_game(id);

    match data {
        Ok((id, _, _type, is_over, answer, guesses)) => {
            println!("ID:      {}", id);
            println!("Type:    {}", _type);
            println!("Status:  {}", if is_over { "Finished" } else { "Active" });
            println!("Answer:  {}", answer);
            println!("Guesses: {}", guesses.unwrap_or_else(|| "None".to_string()));
        },
        Err(_) => println!("Game {} not found", id),
    }
}