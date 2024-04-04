# Fibonacci Calculator

This Rust program calculates the Fibonacci number for a given input. It prompts the user to enter a number and then computes the corresponding Fibonacci number.

## How to Use

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo run` to compile and execute the program.
4. Follow the prompt to enter a number for which you want to calculate the Fibonacci number.

## Usage Example

```bash
$ cargo run

Enter a number to calculate the Fibonacci:
$ 10

The Fibonacci number for 10 is: 55
```

## Fibonacci Calculation

The Fibonacci number for a given input n is calculated iteratively using the following algorithm:

1. Initialize variables a and b to 0 and 1 respectively.
2. Iterate from 2 to n, calculating the Fibonacci number at each step by adding a and b and updating the variables accordingly.
3. Return the calculated Fibonacci number.