
# Langscript
## Features

### Lexer

* tokenize from source code.
* Supports basic arithmetic operations: `+`, `-`, `*`, `>`, `<` and `/`.
* Supports `()` `{}` paren and braces.
* Supports `,` `.` `;` `*` `!` tokens.
* Supports two char tokens like `!=`
* Supports `//` line comments and `/* */` multi line comments
* Supports String, Number and Identifiers and their operations
* Supports `print`
* Supports Expressions (Assign, Logical, Binary, Group, Literal, Unary, Ternary, Variable)
* Supports scopes with different environment
* Supports `while` and `for` expressions (`for` being a syntactic sugar of while)
* Supports `break` statement
* Supports interpreter
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
do `cargo run` to play with the interpreter and also now you can code, with the command `cargo run [path]` you can run any code you have in any extension.

## Roadmap

- [x]  Add parser.
- [x]  Add Interpreter.
- [x]  Add functionality with `let`
- [x]  Add functionality with `if` and `else`
- [x]  Add functionality with `while`
- [x]  Add functionality with `for`
- [ ]  Add functionality with `fun`
- [ ]  Add functionality with `return`
- [ ]  Add funciontality with `class`
- [ ]  Add functionality with `methods`



## Contributing

Feel free to fork this repository, submit issues, or contribute to the project by creating pull requests.

There are yet things to optimize in the code and also to see the limit cases on whats written in the source code, so feel free to test the code here!