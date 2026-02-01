mod answer;

use std::io;

pub fn new() {
    let answer = answer::generate_answer();
    println!("Answer: {}", answer);
    println!("Game started!");
    let mut input = String::new();
    println!("Please enter a word:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input.trim());
}