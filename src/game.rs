mod answer;

pub fn new() {
    let answer = answer::generate_answer();
    println!("Answer: {}", answer);
    println!("Game started!");
}