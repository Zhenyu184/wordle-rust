use std::time::SystemTime;

pub fn generate_answer() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");
    let mut seed = since_the_epoch.as_secs();
    let mut result = String::new();

    for _ in 0..5 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let char_offset = (seed % 26) as u8;
        result.push((b'a' + char_offset) as char);
    }

    result
}