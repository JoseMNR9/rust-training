pub fn encrypt(input: &str) -> String {
    let normalized_text: String = input.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
  
    let length = normalized_text.len();
    let row_length = f64::sqrt(length as f64).ceil() as usize;
    let column_length = (length as f64 / row_length as f64).ceil() as usize;
  
    let mut rectangle: Vec<String> = vec![];
    for i in 0..column_length {
        let start = i * row_length;
        let end = (start + row_length).min(length);
        rectangle.push(normalized_text[start..end].to_string());
    }
  
    let mut encoded_message = String::new();
    for i in 0..row_length {
        for j in 0..column_length {
            if let Some(c) = rectangle.get(j).and_then(|s| s.chars().nth(i)) {
                encoded_message.push(c);
            } else {
                // Add space for empty characters in the rectangle
                encoded_message.push(' ');
            }
        }
        if i < row_length - 1 {
            encoded_message.push(' ');
        }
    }
  
    encoded_message
  }
  

fn main() {
    println!("Enter the phrase to encrypt:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    input = input.trim_end().to_string();
    
    let encrypted_message = encrypt(&input);
    
    println!("Encrypted phrase: {}", encrypted_message);
}
