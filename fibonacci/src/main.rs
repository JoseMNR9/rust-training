use std::io;

fn main() {
    // Prompt the user to enter a number to calculate Fibonacci
    println!("Enter a number to calculate the Fibonacci:");

    // Create a mutable string to store user input
    let mut number = String::new();
  
    // Read the user input
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");

    // Parse the input string into a u32
    let number: u32 = match number.trim().parse() {
        Ok(num) => num, // If parsing successful, assign the value to number
        Err(err) => { // If parsing fails, print an error message and exit the program
            println!("Invalid input: {}", err);
            return;
        }
    };
  
    // Call the fibonacci function with the input number
    fibonacci(number);
}

// Function to calculate the Fibonacci number
fn fibonacci(n: u32) -> u32 {
    // Base case: return n if it's 0 or 1
    if n <= 1 {
      return n;
    }
  
    // Initialize Fibonacci sequence variables
    let mut a = 0;
    let mut b = 1;
  
    // Iterate from 2 to n to calculate Fibonacci number
    for _ in 2..=n {
      let c = a + b;
      a = b;
      b = c;
    }

    // Print the Fibonacci number for the input n
    println!("The Fibonacci number for {} is: {}", n, b);
  
    // Return the calculated Fibonacci number
    b
}
