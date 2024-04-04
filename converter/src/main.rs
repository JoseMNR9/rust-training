use std::io;

fn main() {
    println!("Select to convert Fahrenheit to Celsius (f) or Celsius to Fahrenheit (c):");
  
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature");
  
    let temp = temp.trim(); // Remove trailing whitespace from user input
  
    if temp == "f" {
        // Add conversion logic here for Fahrenheit to Celsius
        let temp_fah = read_farenheit_temperature();
        let temp_cel = fahrenheit_to_celsius(temp_fah);
        println!("{}°F is equivalent to {}°C", temp_fah, temp_cel);
    } else if temp == "c" {
        // Add conversion logic here for Celsius to Fahrenheit
        let temp_cel = read_celsius_temperature();
        let temp_fah = celsius_to_fahrenheit(temp_cel);
        println!("{}°C is equivalent to {}°F", temp_cel, temp_fah);
    } else {
        println!("Invalid input. Please enter 'f' or 'c'.");
    }
  }
  
  // Implement functions to read Fahrenheit and Celsius temperatures from user input
  fn read_farenheit_temperature() -> f32 {
    loop {
        println!("Enter temperature in Fahrenheit:");
        let mut temp_str = String::new();
        io::stdin()
            .read_line(&mut temp_str)
            .expect("Failed to read temperature");
    
        let temp_str = temp_str.trim(); // Remove whitespace
    
        // Attempt to parse the input string to an f32 value
        match temp_str.parse::<f32>() {
          Ok(temp_fah) => return temp_fah,
          Err(_) => println!("Invalid input. Please enter a number."),
        }
      }
  }
  
  fn read_celsius_temperature() -> f32 {
    loop {
        println!("Enter temperature in Celsius:");
        let mut temp_str = String::new();
        io::stdin()
            .read_line(&mut temp_str)
            .expect("Failed to read temperature");
    
        let temp_str = temp_str.trim(); // Remove whitespace
    
        // Attempt to parse the input string to an f32 value
        match temp_str.parse::<f32>() {
          Ok(temp_cel) => return temp_cel,
          Err(_) => println!("Invalid input. Please enter a number."),
        }
      }
  }
  
  // Implement conversion functions (replace with your conversion logic)
  fn fahrenheit_to_celsius(temp_fah: f32) -> f32 {
    (temp_fah - 32.0) * 5.0 / 9.0
  }
  
  fn celsius_to_fahrenheit(temp_cel: f32) -> f32 {
    (temp_cel * 9.0 / 5.0) + 32.0
  }
  
