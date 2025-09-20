use std::vec;

use crate::frontend::{ast::{BinaryOp, Expr, Program, UnaryOp}, tokens::{Span, Token}};

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    position: usize,
    current: (Token, Span),
    errors: Vec<ParserError>
}

#[derive(Debug)]
pub struct ParserError {
    message: String,
    span: Span,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {

        let current = tokens
            .first()
            .cloned()
            .unwrap_or_else(|| (Token::EOF, Span { start: 0, end: 0, line: 0, column: 0 }));

        Self {
            tokens,
            position: 0,
            current,
            errors: vec![]
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, Vec<ParserError>> {

        let mut expressions = vec![];

        while !self.is_at_end() {
            match self.expression() {
                Ok(expr) => {
                    expressions.push(expr);

                    // if !self.is_at_end() {
                    //     if let Err(err) = self.consume(Token::Semicolon, "Expect ';' after expression") {
                    //         self.errors.push(err);
                    //         self.synchronize();
                    //     }
                    // }
                },
                Err(err) => {

                    // self.errors.push(err);
                    // self.synchronize();
                    
                    // // Try to continue parsing after error recovery
                    // if self.is_at_end() {
                    //     break;
                    // }
                }
            }
        }
        
        if self.errors.is_empty() {
            Ok(expressions)
        } else {
            Err(std::mem::take(&mut self.errors))
        }

    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while matches!(self.current.0, Token::DoubleEquals | Token::BangEquals) {

            let op = match &self.current.0 {
                Token::DoubleEquals => BinaryOp::Equals,
                Token::BangEquals => BinaryOp::NotEquals,
                _ => unreachable!()
            };

            self.advance();

            let right = Box::from(self.comparison()?);

            expr = Expr::Binary {
                left: Box::from(expr),
                op,
                right
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        
        while matches!(self.current.0, Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual) {

            let op = match &self.current.0 {
                Token::Greater => BinaryOp::GreaterThan,
                Token::GreaterEqual => BinaryOp::GreaterEq,
                Token::Less => BinaryOp::LessThan,
                Token::LessEqual => BinaryOp::LessEq,
                _ => unreachable!()
            };

            self.advance();

            let right = Box::from(self.term()?);

            expr = Expr::Binary {
                left: Box::from(expr),
                op,
                right
            }

        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while matches!(self.current.0, Token::Minus | Token:: Plus) {

            let op = match &self.current.0 {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };

            self.advance();

            let right = Box::from(self.factor()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
            };

        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while matches!(self.current.0, Token::Star | Token::Slash) {

            let op = match &self.current.0 {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                _ => unreachable!(),
            };

            self.advance();

            let right = Box::from(self.unary()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
            };
            println!("{:#?}", expr);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {

        if matches!(self.current.0, Token::Bang | Token::Minus) {

            let op = match &self.current.0 {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                Token::BitwiseNot => UnaryOp::BitNot,
                _ => unreachable!()
            };

            self.advance();

            let expr = self.unary()?;

            Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
            })

        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {

        match self.current.0.clone() { // TODO: clone
            Token::IntLiteral(n) => {
                self.advance();
                return Ok(Expr::IntLiteral(n))
            },
            Token::FloatLiteral(f) => {
                self.advance();
                return Ok(Expr::FloatLiteral(f))
            },
            Token::CharLiteral(c) => {
                self.advance();
                return Ok(Expr::CharLiteral(c))
            }
            Token::StringLiteral(s) => {
                self.advance();
                return Ok(Expr::StringLiteral(s))
            },
            Token::BoolLiteral(b) => {
                self.advance();
                if b {
                    return Ok(Expr::BoolLiteral(true))
                } else {
                    return Ok(Expr::BoolLiteral(false))
                }
            },
            Token::Identifier(name) => {
                self.advance();
                return Ok(Expr::Identifier(name))
            },
            Token::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(Token::RParen, "Expect ')' after expression.")?;
                return Ok(Expr::Grouped(Box::from(expr)))
            },
            _ => return Err(self.error("Expect expression."))
        }
    }

    // Helper functions

    fn advance(&mut self) {
        self.position += 1;
        if self.position < self.tokens.len() {
            self.current = self.tokens[self.position].clone();
        } else {
            self.current = (Token::EOF, Span { 
                start: self.current.1.end, 
                end: self.current.1.end, 
                line: self.current.1.line, 
                column: self.current.1.column 
            });
        }
    }

    fn current_token(&self) -> &Token {
        &self.current.0
    }

    fn current_span(&self) -> &Span {
        &self.current.1
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::EOF)
    }

    fn peek_token(&self) -> Option<&(Token, Span)> {
        if self.position + 1 < self.tokens.len() {
            Some(&self.tokens[self.position + 1])
        } else {
            None
        }
    }

    fn check(&self, token: Token) -> bool {
        self.current_token() == &token
    }

    fn matches(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token: Token, error_msg: &str) -> Result<(), ParserError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(error_msg))
        }
    }

    fn error(&self, message: &str) -> ParserError {
        ParserError {
            message: message.to_string(),
            span: self.current_span().clone(),
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        
        while !self.is_at_end() {
            if let Some(prev) = self.previous_token() {
                if matches!(prev.0, Token::Semicolon) {
                    return;
                }
            }
            
            match self.current_token() {
                Token::Fn | Token::Let | Token::Struct | Token::Enum 
                | Token::If | Token::While | Token::For | Token::Return => return,
                _ => self.advance(),
            }
        }
    }

    fn previous_token(&self) -> Option<&(Token, Span)> {
        if self.position > 0 {
            Some(&self.tokens[self.position - 1])
        } else {
            None
        }
    }

    fn next_token(&self) -> Option<&(Token, Span)> {
        self.tokens.iter().nth(self.position)
    }
}