# Number Guessing Game

This Rust program is a simple number guessing game. It generates a random number between 1 and 100 (inclusive) and prompts the user to guess that number.

## How to Use

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo run` to compile and execute the program.
4. Follow the prompts to guess the randomly generated number.

## Usage Example

```bash
$ cargo run

Guess the number!
Please input your guess.
$ 50

You guessed: 50
Too big!
Please input your guess.
$ 25

You guessed: 25
Too small!
Please input your guess.
$ 37

You guessed: 37
You win!
```

## Game Logic

1. The program generates a random number between 1 and 100 using the rand crate.
2. It prompts the user to input their guess.
3. The user's guess is compared with the secret number:
    - If the guess is less than the secret number, the program outputs "Too small!".
    - If the guess is greater than the secret number, the program outputs "Too big!".
    - If the guess is equal to the secret number, the program outputs "You win!" and exits the loop.
4. The loop continues until the user guesses the correct number.