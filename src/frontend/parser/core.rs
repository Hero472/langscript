use crate::frontend::{lexer::{span::Span, tokens::Token}, parser::error::ParserError};

pub struct ParserCore {
    pub tokens: Vec<(Token, Span)>,
    pub position: usize,
    pub current: (Token, Span),
    pub errors: Vec<ParserError>,
    filename: String,
}

impl ParserCore {

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

    pub fn advance(&mut self) {
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

    pub fn current_token(&self) -> &Token {
        &self.current.0
    }

    pub fn current_span(&self) -> &Span {
        &self.current.1
    }

    pub fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::EOF)
    }

    pub fn peek_token(&self) -> Option<&(Token, Span)> {
        if self.position + 1 < self.tokens.len() {
            Some(&self.tokens[self.position + 1])
        } else {
            None
        }
    }

    pub fn actual_token(&self) -> &(Token, Span) {
        &self.tokens[self.position]
    }

    pub fn check(&self, token: Token) -> bool {
        self.current_token() == &token
    }

    pub fn matches(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn consume(&mut self, token: Token, error_msg: &str) -> Result<(), ParserError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(error_msg))
        }
    }

    pub fn error(&self, message: &str) -> ParserError { // add span in the method so it always tells me where is it
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

    pub fn synchronize(&mut self) {
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

    pub fn previous_token(&self) -> Option<&(Token, Span)> {
        if self.position > 0 {
            Some(&self.tokens[self.position - 1])
        } else {
            None
        }
    }

    pub fn next_token(&self) -> Option<&(Token, Span)> {
        self.tokens.iter().nth(self.position)
    }

    pub fn take_errors(&self) -> Vec<ParserError> {
        self.errors.clone()
    }

}