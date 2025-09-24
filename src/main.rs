use std::{error::Error, fs, process};
use langscript::{frontend::{lexer::lexer::Lexer, parser::Parser}, middle::typeck::TypeChecker, vm::VirtualMachine};

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
    for (token, span) in &tokens {
        println!("  {:?} at {:?}", token, span);
    }

    println!();

    let mut parser = Parser::new(tokens, "testing.lss".to_string());

    let program = parser.parse();

    if program.is_err() {
        let errors = program.unwrap_err();
        for error in &errors {
            println!("Parser Error: {:#?}", error);
        }
        println!("Total errors: {}", errors.len());
        process::exit(1);
    }

    let program = program.unwrap();

    println!("Parsed expressions:");
    for decl in &program.declarations {
        println!("{:#?}", decl);
    }
    println!("Number of expressions: {}", &program.declarations.len());

    let mut type_check = TypeChecker::new();

    let typed_errors = type_check.check(&program);

    if typed_errors.is_err() {
        let errors = typed_errors.unwrap_err();
        for type_error in &errors {
            println!("Type Error: {}", type_error);
        }
        println!("Total type errors: {}", errors.len());
        process::exit(1);
    }

    let mut vm = VirtualMachine::new();

   let result = vm.interpret(program);

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