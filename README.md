# Rust CLI Calculator

A simple command-line calculator built with Rust.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

What things you need to install and how to install them

- Rust: Follow the installation guide on the [official website](https://www.rust-lang.org/tools/install)

### Installing

A step by step series of examples that tell you how to get a development environment running


1. Clone the repository
   ```
   git clone https://github.com/inode-001/simple_rust_calculator.git
   ```
2. Build the project
   ```
   cargo build --release
   ```
3. Run the calculator
   ```
   ./target/release/cli-calculator
   ```
# Usage

1. Enter the first number.
2. Enter the second number.
3. Enter the operator (`+`, `-`, `*`, `/`) or `q` to quit.
4. The result will be displayed, and you can continue to perform more calculations or quit.

# Example Interaction


Enter first number:
10
Enter second number:
5
Enter operator (+, -, *, /) or 'q' to quit:
+
The result is: 15

Enter first number:
20
Enter second number:
4
Enter operator (+, -, *, /) or 'q' to quit:
/
The result is: 5

Enter first number:
8
Enter second number:
0
Enter operator (+, -, *, /) or 'q' to quit:
/
Error: Division by zero

Enter first number:
5
Enter second number:
5
Enter operator (+, -, *, /) or 'q' to quit:
q
Quitting...



- `Cargo.toml`: Contains the project metadata and dependencies.
- `src/main.rs`: The main program file containing the implementation of the  calculator.
- src/lib.rs : The library file of the calculator contains code required by main.rs
