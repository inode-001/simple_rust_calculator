
CLI Calculator

   - A simple command-line interface (CLI) calculator written in Rust. This calculator can perform basic arithmetic operations: addition, subtraction, multiplication, and division. It continuously prompts the user for input and performs the requested operations until the user decides to quit.

  Features

- Perform addition, subtraction, multiplication, and division
- Handles invalid input gracefully
- Simple and easy-to-use interface

 Getting Started

# Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) programming language installed on your system.

# Installation

1. Clone the repository:
   git clone https://github.com/inode-001/simple_rust_calculator.git
   cd cli_calculator

2. Build the project:
   cargo build
   ```

# Running the Calculator

To run the calculator, use the following command:
cargo run

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
```

# Project Structure

```
cli_calculator/
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

- `Cargo.toml`: Contains the project metadata and dependencies.
- `src/main.rs`: The main program file containing the implementation of the  calculator.
- src/lib.rs : The library file of the calculator contains code required by main.rs



## Acknowledgments

- Thanks to the Rust community for their helpful documentation and resources.

