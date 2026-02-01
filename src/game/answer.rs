use std::time::SystemTime;

pub fn generate_answer(length: u8) -> String {
    let content = include_str!("../../words/words.txt");
    let words: Vec<&str> = content
        .lines()
        .map(|line| line.trim())
        .filter(|w| w.len() == length as usize)
        .collect();

    if words.is_empty() {
        return "apple".to_string();
    }

    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    let seed = since_the_epoch.as_nanos();
    let index = (seed as usize) % words.len();

    words[index].to_string()
}