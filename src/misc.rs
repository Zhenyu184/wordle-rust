use base64::{Engine as _, engine::general_purpose};

pub fn encrypt(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn decrypt(input: &str) -> String {
    match general_purpose::STANDARD.decode(input) {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| input.to_string()),
        Err(_) => input.to_string(),
    }
}