Here's a simple and clear README file for your Rust Calculator project:

---

# Rust Calculator

This is a command-line calculator application written in Rust. The calculator supports basic arithmetic operations as well as calculating the square root of a number. The results are displayed in a structured table format for better readability.

## Features

- Addition
- Subtraction
- Multiplication
- Division (with error handling for division by zero)
- Square root calculation (with error handling for negative numbers)
- User-friendly, table-formatted output

## Getting Started

### Prerequisites

To run this program, you need to have the Rust programming language installed. If you don’t have it installed, follow these instructions:

1. Visit [rust-lang.org](https://www.rust-lang.org/tools/install)
2. Follow the installation guide for your operating system.

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/t9fiction/Rust-Calculator.git
   ```
2. Navigate to the project directory:
   ```bash
   cd rust-calculator
   ```
3. Compile and run the program:
   ```bash
   cargo run
   ```

## Usage

When you start the program, it will display a menu with options for different mathematical operations. Here’s a breakdown of how to use the calculator:

1. Choose the operation you want to perform by entering the corresponding number (1-5).
2. For operations 1-4 (Addition, Subtraction, Multiplication, Division), you’ll be prompted to enter two numbers.
3. For the Square Root option (5), you’ll enter a single number.
4. The program will then display the result in a table format.
5. Enter `0` to exit the calculator.

Example of an operation:

```
+-----------------------------------+
| Choose the operation to perform   |
+-----------------------------------+
| 1. Addition                       |
| 2. Subtraction                    |
| 3. Multiplication                 |
| 4. Division                       |
| 5. Square root                    |
| 0. Exit                           |
+-----------------------------------+
Enter your choice: 1
Enter the first number: 10
Enter the second number: 5

+-----------------------------------+
|            Result                 |
+-----------------------------------+
| Operation: Addition               |
| Result: 15                        |
+-----------------------------------+
```

## Error Handling

- If the user enters an invalid input (non-number), the program will prompt the user to try again.
- For division, if the second number is `0`, the program will notify the user that division by zero is not allowed.
- For square root, if the input number is negative, the program will notify the user that the square root of a negative number is undefined.

## Contributing

If you’d like to contribute to this project, feel free to fork the repository and submit a pull request.

## License

This project is open-source and licensed under the MIT License.

---
