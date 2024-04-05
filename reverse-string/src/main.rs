use std::io;

fn main() {
    println!("Please enter a word:");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    let word = word.trim();

    let reversed_word: String = word.chars().rev().collect();

    println!("Reversed word: {}", reversed_word);
}
