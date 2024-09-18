
# Langscript
## Features

### Lexer

* tokenize from source code.
* Supports basic arithmetic operations: `+`, `-`, `*`, `>`, `<` and `/`.
* Supports `()` `{}` paren and braces.
* Supports `,` `.` `;` `*` `!` tokens.
* Supports two char tokens like `!=`
* Supports `//` line comments and `/* */` multi line comments
* Supports String, Number and Identifiers
### Prerequisites

Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/)

### Installation

1. Clone this repository
```bash
git clone https://github.com/Hero472/langscript.git
cd langscript
```
2. Build the project
```bash
cargo build
```
3. Run the project
```bash
cargo run
```

4. Test the project
```bash
cargo test
```


### Usage
For now it doesn't has a very good way of using it more than using tests for testing source code, unless you use `cargo run` to run in the terminal (yet to implement more functionality)

## Roadmap

- [ ]  Add parser.
- [ ]  Add Interpreter.
- [ ]  Add functionality with `let`
- [ ]  Add functionality with `if` and `else`
- [ ]  Add functionality with `while`
- [ ]  Add functionality with `for`
- [ ]  Add functionality with `fun`



## Contributing

Feel free to fork this repository, submit issues, or contribute to the project by creating pull requests.

There are yet things to optimize in the code and also to see the limit cases on whats written in the source code, so feel free to test the code here!