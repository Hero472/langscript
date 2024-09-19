use std::{env, fs};
use std::io::{self,BufRead, Write};
use std::process::exit;

use parser::Parser;

use crate::lexer::*;

mod lexer;
mod generate_ast;
mod parser;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage langscript");
        exit(64)
    } else if args.len() == 2 {
         match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error:\n{}",msg);
                exit(1);
            }
         }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error:\n{}",msg);
                exit(1);
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), String>{
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run_prompt() -> Result<(), String> {

    loop {
        print!(">");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not flush stdout".to_string())
        }
        let mut buffer: String = String::new();
        let stdin: io::Stdin = io::stdin();
        let mut handle: io::StdinLock<'_> = stdin.lock();

        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    return Ok(())
                }
            },
            Err(_) => return Err("Could not read line".to_string())
        }
        print!("< {}",buffer);

        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}",msg)
        }

    }

}

fn run(contents: &str) -> Result<(),String> {
    let mut lexer: Lexer = Lexer::new(contents);
    let tokens: Vec<Token> = lexer.scan_tokens()?;
    let mut parser: Parser = Parser::new(tokens); 
    let expr: generate_ast::Expr = parser.parse()?;

    return Ok(());

}