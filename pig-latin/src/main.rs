fn to_pig_latin(text: &str) -> String {
    let mut pig_latin = String::new();

    for word in text.split_whitespace() {
        let mut first_consonant_cluster = String::new();
        let mut remaining_letters = String::new();

        // Handle UTF-8 characters
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();

        // Separate consonant cluster (if any)
        for ch in chars {
            if is_vowel(ch) {
                remaining_letters.push(ch);
                break;
            } else {
                first_consonant_cluster.push(ch);
            }
        }

        // Construct Pig Latin word
        pig_latin.push_str(&remaining_letters);
        pig_latin.push_str("-");
        pig_latin.push_str(&first_consonant_cluster);

        if is_vowel(first_char) {
            pig_latin.push_str("hay");
        } else {
            pig_latin.push_str("ay");
        }
        pig_latin.push_str(" ");
    }

    pig_latin.trim().to_string() // Remove trailing whitespace
}

fn is_vowel(ch: char) -> bool {
    match ch.to_lowercase().to_string().as_str() {
        "a" | "e" | "i" | "o" | "u" => true,
        _ => false,
    }
}


fn main() {
    let mut input = String::new();
    println!("Enter a sentence to convert to Pig Latin:");

    // Get user input with UTF-8 support
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let pig_latin_text = to_pig_latin(&input.trim()); // Remove trailing newline

    println!("The sentence in Pig Latin is: {}", pig_latin_text);
}

