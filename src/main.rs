use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;
mod interpreter;

fn main() {
    let input: String = "let x = 4 + 3".to_string();
    let lexer: Lexer = Lexer::new(input);
    let mut parser: Parser = Parser::new(lexer);

    let ast: parser::ASTNode = parser.parse_statement();
    println!("Parsed AST: {:?}", ast);

    let mut interpreter: Interpreter = Interpreter::new();
    let result: f64 = interpreter.interpret(ast);
    println!("Result: {}", result);
}