use std::{error::Error, fs};
use langscript::{frontend::{lexer::lexer::Lexer, parser::Parser}, middle::typeck::TypeChecker, vm::{interpreter::Interpreter, VirtualMachine}};

fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.world>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename)?;

    let mut lexer = Lexer::new(&source, "testing.lss");
    let tokens = lexer.tokenize();

    println!("Tokens:");
    for (token, span) in tokens.clone() {
        println!("  {:?} at {:?}", token, span);
    }

    println!();

    let mut parser = Parser::new(tokens, "testing.lss".to_string());

    let exprs = parser.parse();

    if exprs.is_err() {
        let errors = exprs.unwrap_err();
        for error in &errors {
            println!("Parser Error: {:#?}", error);
        }
        println!("Total errors: {}", errors.len());
        return Ok(());
    }

    let exprs = exprs.unwrap();

    println!("Parsed expressions:");
    for expr in &exprs {
        println!("{:#?}", expr);
    }
    println!("Number of expressions: {}", exprs.len());

    let mut type_check = TypeChecker::new();

    let typed_errors = type_check.check(exprs.clone());

    if typed_errors.clone().is_err() {
        for type_error in typed_errors.unwrap_err() {
            println!("{}", type_error)
        }
    }

    let mut vm = VirtualMachine::new();

   let result = vm.interpret(exprs);

   match result {
        Ok(value) => {
            println!("\nEvaluation successful!");
            println!("Result: {:#?}", value);
        }
        Err(error) => {
            println!("\nRuntime error occurred:");
            println!("Error: {:#?}", error);
        }
    }
    
    Ok(())
}