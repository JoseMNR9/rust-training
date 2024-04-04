# Pig Latin Converter

This Rust program converts a given sentence into Pig Latin. Pig Latin is a language game that alters English words by moving the initial consonant or consonant cluster of each word to the end of the word and appending "ay" or "hay".

## How to Use

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo run` to compile and execute the program.
4. Enter a sentence when prompted.
5. The program will output the sentence converted into Pig Latin.

## Usage Example

```bash
$ cargo run

Enter a sentence to convert to Pig Latin:
$ Hello world

The sentence in Pig Latin is: ello-Hay orld-way
```

## Conversion Logic

- The program iterates through each word in the input sentence.
- For each word, it separates the initial consonant cluster (if any) and the remaining letters.
- It constructs the Pig Latin word by appending the remaining letters, the consonant cluster, and the appropriate suffix ("ay" if the first letter is a consonant, "hay" if the first letter is a vowel).
- The converted words are then joined together to form the final Pig Latin sentence.
