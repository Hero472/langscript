use crate::frontend::{lexer::{span::Span, tokens::Token}, parser::{ast::{Expr, Program, Stmt}, core::ParserCore, declarations::DeclarationParser, error::ParserError, expressions::ExpressionParser, statements::StatementParser}};

pub mod error;
pub mod ast;
pub mod core;
pub mod expressions;
pub mod statements;
pub mod declarations;

pub struct Parser {
    core: ParserCore
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>, filename: String) -> Self {
        Self {
            core: ParserCore::new(tokens, filename),
        }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut declarations = vec![];

        while !self.core.is_at_end() {
            match self.core.declaration() {
                Ok(expr) => {
                    declarations.push(expr);

                    if !self.core.is_at_end() {
                        if let Err(err) = self.core.consume(Token::Semicolon, "Expect ';' after expression") {
                            self.core.errors.push(err);
                            self.core.synchronize();
                        }
                    }
                },
                Err(err) => {
                    self.core.errors.push(err);
                    self.core.synchronize();
                    
                    if self.core.is_at_end() {
                        break;
                    }
                }
            }
        }
        
        if self.core.errors.is_empty() {
            Ok(Program {declarations})
        } else {
            Err(self.core.take_errors())
        }
    }

}