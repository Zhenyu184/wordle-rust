use crate::database;

#[derive(Clone, Copy, PartialEq)]
enum Color {
    Green,
    Yellow,
    Reset,
}

impl Color {
    fn as_str(&self) -> &'static str {
        match self {
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Reset => "\x1b[0m",
        }
    }
}

fn get_guess_colors(guess: &str, answer: &str, width: usize) -> Vec<Color> {
    let guess_chars: Vec<char> = guess.chars().collect();
    let answer_chars: Vec<char> = answer.chars().collect();
    let mut colors = vec![Color::Reset; width];
    let mut answer_used = vec![false; width];

    // correct position(green)
    for i in 0..width {
        if i >= guess_chars.len() || i >= answer_chars.len() {
            continue;
        }

        if guess_chars[i] == answer_chars[i] {
            colors[i] = Color::Green;
            answer_used[i] = true;
        }
    }

    // wrong position(yellow)
    for i in 0..width {
        if colors[i] != Color::Reset || i >= guess_chars.len() {
            continue;
        }

        let char = guess_chars[i];
        if let Some(idx) = answer_chars.iter().enumerate().position(|(j, &c)| !answer_used[j] && c == char) {
            colors[i] = Color::Yellow;
            answer_used[idx] = true;
        }
    }
    
    colors
}

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
            let colors = get_guess_colors(guess, answer, width);

            for (i, char) in guess.chars().enumerate() {
                let color = if i < width { colors[i] } else { Color::Reset };
                print!("[ {}{}{} ]", color.as_str(), char.to_ascii_uppercase(), Color::Reset.as_str());
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
            draw_board(_type, guesses, &answer);
        },
        Err(_) => println!("Game {} not found", id),
    }
}