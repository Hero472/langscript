
# Rust Lexer, Parser, and interpreter

This project implements a simple programming language in Rust with the following components:

* **Lexer**: Tokenizes input strings into meaningful tokens.
* **Parser**: Converts tokens into an Abstract Syntax Tree (AST).
* **Interpreter:** Evaluates the AST and returns results.

## Features

* Supports basic arithmetic operations: `+`, `-`, `*`, and `/`.
* Supports variable assignments with `let`.
* Parses and evaluates expressions involving numbers, variables, and operations.
* Handles grouping via parentheses `()`.
### Prerequisites

Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/)

### Installation

1. Clone this repository
```bash
git clone https://github.com/Hero472/langscript.git
cd rust-lexer-parser-interpreter
```
2. Build the project
```bash
cargo build
```



### Usage
You can run the project with sample input using `cargo run`:
```bash
cargo run
```
## Example input

The project currently handles two types of statements: expressions and assignments.
* **Expression**: Input `3 + 5 * (10 - 4)`
```
cargo run
```
Output:
```bash
Parsed AST: BinaryOperation { left: Number(3.0), operator: "+", right: BinaryOperation { left: Number(5.0), operator: "*", right: Grouping(BinaryOperation { left: Number(10.0), operator: "-", right: Number(4.0) }) } }
Result: 33
```

* **Variable Assignment**: `let x = 10 + 2`
```
cargo run
```

Output:

```bash
Parsed AST: Assignment("x", BinaryOperation { left: Number(10.0), operator: "+", right: Number(2.0) })
Result: 12
```

## Roadmap

- [ ]  Extend the interpreter to support conditional expressions (`if`, `else`) and loops.


