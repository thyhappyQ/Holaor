# Holaor

A simple Rust tool that reads and validates Chinese ID cards, extracting key information like age and gender.

## Usage

### Build the project

```bash
cargo build --release
```

### Run the program

Pass your ID card number as an argument:

```bash
cargo run --release -- "Your18DigitIDNumber"
```

### Example

```bash
cargo run --release -- "110101199003076619"
```

### Output

The program will display:
- **ID card format validation** - Checks if the ID card is valid (18 digits)
- **Age** - Calculated from the ID card's birth date
- **Gender** - Determined from the ID card's check digit

The output uses colored text:
- 🟢 **Green** for valid/OK results
- 🔴 **Red** for errors or invalid data
- 🟡 **Yellow** for warnings

## Requirements

- Rust 1.56+

## How it works

- Extracts birth date from positions 6-14 of the ID card
- Calculates age based on current date
- Determines gender from the 17th digit (odd = male, even = female)
- Validates that the ID card is exactly 18 digits long
