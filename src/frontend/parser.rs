use crate::frontend::{ast::Program, tokens::{Span, Token}};

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
    pub fn new(&self, tokens: Vec<(Token, Span)>) -> Self {

        let current = tokens
            .first()
            .cloned()
            .unwrap_or_else(|| (Token::EOF, Span { start: 0, end: 0, line: 0, column: 0 }));

        Self {
            tokens,
            position: 1,
            current,
            errors: vec![]
        }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut declarations = Vec::new();

        todo!()
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