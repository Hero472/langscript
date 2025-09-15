use std::{error::Error, fs};
use langscript::frontend::lexer::Lexer;

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
    for (token, span) in tokens {
        println!("  {:?} at {:?}", token, span);
    }
    
    Ok(())
}