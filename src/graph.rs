use crate::database;


fn draw_board(game_type: u8, guesses: Option<String>) {
    let (width, height) = if game_type == 56 { (5, 6) } else { (5, 6) };

    let guess_list: Vec<&str> = match &guesses {
        Some(s) if !s.is_empty() => s.split(',').collect(),
        _ => Vec::new(),
    };

    for row in 0..height {
        print!("Row{}: ", row + 1);
        if row < guess_list.len() {
            let guess = guess_list[row];
            for char in guess.chars() {
                print!("[ {} ]", char.to_ascii_uppercase());
            }
        } else {
            for _ in 0..width {
                print!("[   ]");
            }
        }
        println!();
    }
}

pub fn show(id: i64) {
    let db = database::Database::connect();
    let data = db.get_game(id);

    match data {
        Ok((id, _, _type, is_over, answer, guesses)) => {
            println!("ID:      {}", id);
            println!("Type:    {}", _type);
            println!("Status:  {}", if is_over { "over" } else { "active" });
            println!("Answer:  {}", answer);
            draw_board(_type, guesses);
        },
        Err(_) => println!("Game {} not found", id),
    }
}