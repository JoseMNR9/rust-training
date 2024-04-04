# Temperature Converter

This simple Rust program converts temperatures between Fahrenheit and Celsius. It provides a command-line interface for users to select the type of conversion they want and input temperatures to convert.

## How to Use

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo run` to compile and execute the program.
4. Follow the prompts to select the conversion type and input the temperature to convert.

## Usage Example

```bash
$ cargo run

Select to convert Fahrenheit to Celsius (f) or Celsius to Fahrenheit (c):
$ c

Enter temperature in Celsius:
$ 20

20°C is equivalent to 68°F
```

## Conversion Logic

- To convert Fahrenheit to Celsius: (temp_fah - 32.0) * 5.0 / 9.0
- To convert Celsius to Fahrenheit: (temp_cel * 9.0 / 5.0) + 32.0