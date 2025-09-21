use std::{error::Error, fs};
use langscript::{frontend::{lexer::Lexer, parser::Parser}, vm::interpreter::Interpreter};

fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.world>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename)?;

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    println!("Tokens:");
    for (token, span) in tokens.clone() {
        println!("  {:?} at {:?}", token, span);
    }

    println!();

    let mut parser = Parser::new(tokens);

    let exprs_print = parser.parse();

    if exprs_print.is_ok() {
        let exprs_print = exprs_print.unwrap();
        for expr in &exprs_print {
            println!("{:#?}", expr)
        }
        println!("{:?}", exprs_print.len())
    } else {
        let exprs_print = exprs_print.unwrap_err();
        for expr in &exprs_print {
            println!("{:#?}", expr)
        }
        println!("{:?}", exprs_print.len())
    }

    let exprs = parser.parse();

    let mut interpreter = Interpreter::new();

    // let values = interpreter.evaluate(exprs.unwrap());

    let a: u64 = 5/3;

    let b :u64 = 5;
    
    let c = a - b;

    println!("{}", a);

    Ok(())
}