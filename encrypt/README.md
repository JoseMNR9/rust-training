# Crypto Square

The Crypto Square is a classic method for composing secret messages. It involves encoding an English text into a square grid format.

## Encoding Process:

1. Normalize the input text by removing spaces and punctuation, and converting all characters to lowercase.
2. Determine the dimensions of the square grid. The number of rows (r) and columns (c) should satisfy the following conditions:
    - r * c should be greater than or equal to the length of the normalized message.
    - c should be greater than or equal to r.
    - c - r should be less than or equal to 1.
3. Organize the normalized characters into rows to form a rectangle.
4. Read the characters column by column, left to right, to encode the message.

```bash
$ cargo run

Enter the phrase to encrypt:
$ hello world
Encrypted phrase: hol ewd lo  lr 
```