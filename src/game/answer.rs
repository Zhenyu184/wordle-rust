use std::time::SystemTime;

pub fn generate(length: u8) -> String {
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
    let epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    let seed = epoch.as_nanos();
    let index = (seed as usize) % words.len();

    words[index].to_string()
}