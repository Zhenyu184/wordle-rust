use crate::database;

fn draw_board(game_type: u8, guesses: Option<String>, answer: &str) {
    let (width, height) = if game_type == 56 { (5, 6) } else { (5, 6) };

    let guess_list: Vec<&str> = match &guesses {
        Some(s) if !s.is_empty() => s.split(',').collect(),
        _ => Vec::new(),
    };

    for row in 0..height {
        print!("Row{}: ", row + 1);
        if row < guess_list.len() {
            let guess = guess_list[row];
            
            let guess_chars: Vec<char> = guess.chars().collect();
            let answer_chars: Vec<char> = answer.chars().collect();
            let mut colors = vec!["\x1b[0m"; width];
            let mut answer_used = vec![false; width];

            // green (correct position)
            for i in 0..width {
                if i < guess_chars.len() && i < answer_chars.len() {
                    if guess_chars[i] == answer_chars[i] {
                        colors[i] = "\x1b[32m";
                        answer_used[i] = true;
                    }
                }
            }

            // orange (wrong position)
            for i in 0..width {
                if colors[i] == "\x1b[0m" && i < guess_chars.len() {
                    let char = guess_chars[i];
                    if let Some(idx) = answer_chars.iter().enumerate().position(|(j, &c)| !answer_used[j] && c == char) {
                        colors[i] = "\x1b[33m";
                        answer_used[idx] = true;
                    }
                }
            }

            for (i, char) in guess.chars().enumerate() {
                let color = if i < width { colors[i] } else { "\x1b[0m" };
                print!("[ {}{}\x1b[0m ]", color, char.to_ascii_uppercase());
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
    let data = db.get_status(id);

    match data {
        Ok((id, _, _type, is_over, answer, guesses)) => {
            println!("ID:      {}", id);
            println!("Type:    {}", _type);
            println!("Status:  {}", if is_over { "over" } else { "active" });
            println!("Answer:  {}", answer);
            draw_board(_type, guesses, &answer);
        },
        Err(_) => println!("Game {} not found", id),
    }
}