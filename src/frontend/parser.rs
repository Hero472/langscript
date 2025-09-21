use std::vec;

use crate::frontend::{ast::{BinaryOp, Expr, UnaryOp}, span::Span, tokens::Token};

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    position: usize,
    current: (Token, Span),
    errors: Vec<ParserError>,
    filename: String
}

#[derive(Debug, Clone)]
pub struct ParserError {
    message: String,
    span: Span,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>, filename: String) -> Self {

        let current = tokens
            .first()
            .cloned()
            .unwrap_or_else(|| (Token::EOF, 
                Span { 
                    start_line: 0,
                    start_column: 0,
                    end_line: 0,
                    end_column: 0,
                    file_id: None 
                }
            ));

        Self {
            tokens,
            position: 0,
            current,
            errors: vec![],
            filename
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

                    self.errors.push(err);
                    self.synchronize();
                    
                    // Try to continue parsing after error recovery
                    if self.is_at_end() {
                        break;
                    }
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

            let combined_span = Span {
                start_line: expr.span().start_line,
                start_column: expr.span().start_column,
                end_line: right.span().end_line,
                end_column: right.span().end_column,
                file_id: expr.span().file_id,
            };

            expr = Expr::Binary {
                left: Box::from(expr),
                op,
                right,
                span: combined_span
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

            let combined_span = Span {
                start_line: expr.span().start_line,
                start_column: expr.span().start_column,
                end_line: right.span().end_line,
                end_column: right.span().end_column,
                file_id: expr.span().file_id,
            };

            expr = Expr::Binary {
                left: Box::from(expr),
                op,
                right,
                span: combined_span
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

            let combined_span = Span {
                start_line: expr.span().start_line,
                start_column: expr.span().start_column,
                end_line: right.span().end_line,
                end_column: right.span().end_column,
                file_id: expr.span().file_id,
            };

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
                span: combined_span
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

            let combined_span = Span {
                start_line: expr.span().start_line,
                start_column: expr.span().start_column,
                end_line: right.span().end_line,
                end_column: right.span().end_column,
                file_id: expr.span().file_id,
            };

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
                span: combined_span
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {

        if matches!(self.current.0, Token::Bang | Token::Minus | Token::Plus) {

            let operator_span = self.current.1;

            let op = match &self.current.0 {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                _ => unreachable!()
            };

            self.advance();

            let expr = self.unary()?;

            let combined_span = Span {
                start_line: operator_span.start_line,
                start_column: operator_span.start_column,
                end_line: expr.span().end_line,
                end_column: expr.span().end_column,
                file_id: operator_span.file_id,
            };

            Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
                span: combined_span
            })

        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {

        match self.current.0.clone() {
            Token::IntLiteral(n) => {
                self.advance();
                return Ok(Expr::IntLiteral(n, *self.current_span()))
            },
            Token::UintLiteral(n) => {
                self.advance();
                return Ok(Expr::UintLiteral(n, *self.current_span()))
            }
            Token::FloatLiteral(f) => {
                self.advance();
                return Ok(Expr::FloatLiteral(f, *self.current_span()))
            },
            Token::CharLiteral(c) => {
                self.advance();
                return Ok(Expr::CharLiteral(c, *self.current_span()))
            }
            Token::StringLiteral(s) => {
                self.advance();
                return Ok(Expr::StringLiteral(s, *self.current_span()))
            },
            Token::BoolLiteral(b) => {
                self.advance();
                if b {
                    return Ok(Expr::BoolLiteral(true, *self.current_span()))
                } else {
                    return Ok(Expr::BoolLiteral(false, *self.current_span()))
                }
            },
            Token::Identifier(name) => {
                self.advance();
                return Ok(Expr::Identifier(name, *self.current_span()))
            },
            Token::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(Token::RParen, "Expect ')' after expression.")?;
                return Ok(Expr::Grouped(Box::from(expr), *self.current_span()))
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

            let eof_span = if let Some(last_token) = self.tokens.last() {

                Span {
                    start_line: last_token.1.end_line,
                    start_column: last_token.1.end_column,
                    end_line: last_token.1.end_line,
                    end_column: last_token.1.end_column,
                    file_id: last_token.1.file_id,
                }
            } else {

                Span {
                    start_line: 1,
                    start_column: 1,
                    end_line: 1,
                    end_column: 1,
                    file_id: None,
                }
            };
            
            self.current = (Token::EOF, eof_span);
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
        if self.is_at_end() {

            let token = self.previous_token().unwrap().clone();

            return ParserError {
                message: format!("{:?} is at end of file", token.0),
                span: token.1
            }
        }
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