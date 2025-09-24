use crate::frontend::{lexer::{span::Span, tokens::Token}, parser::{ast::Stmt, error::ParserError}};

pub struct ParserCore {
    pub tokens: Vec<(Token, Span)>,
    pub position: usize,
    pub current: (Token, Span),
    pub errors: Vec<ParserError>,
    filename: String,
}

impl ParserCore {
    /// Creates a new ParserCore instance with the given tokens and filename.
    /// Initializes the parser state and sets up the first token as current.
    ///
    /// # Arguments
    /// * `tokens` - Vector of token-span pairs representing the tokenized source code
    /// * `filename` - Name of the file being parsed (for error reporting)
    ///
    /// # Examples
    /// ```
    /// let tokens = vec![(Token::Let, span), (Token::Identifier("x".to_string()), span)];
    /// let parser = ParserCore::new(tokens, "example.wld".to_string());
    /// ```
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

    /// Advances to the next token in the token stream.
    /// Updates the current token and handles end-of-file detection.
    /// If at the end of tokens, creates an EOF token with appropriate positioning.
    ///
    /// # Examples
    /// ```ignore
    /// // Before: [Let, Identifier("x"), Semicolon, EOF]
    /// //          ^ current
    /// parser.advance();
    /// // After:  [Let, Identifier("x"), Semicolon, EOF]
    /// //                 ^ current
    /// ```
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


    /// Returns a reference to the current token being processed.
    ///
    /// # Returns
    /// Reference to the current Token
    ///
    /// # Examples
    /// ```ignore
    /// if parser.current_token() == &Token::Let {
    ///     // Handle let statement
    /// }
    /// ```
    pub fn current_token(&self) -> &Token {
        &self.current.0
    }

    /// Returns a reference to the span of the current token.
    /// Useful for error reporting and source location tracking.
    ///
    /// # Returns
    /// Reference to the current token's Span
    pub fn current_span(&self) -> &Span {
        &self.current.1
    }

    /// Checks if the parser has reached the end of the token stream.
    ///
    /// # Returns
    /// `true` if current token is EOF, `false` otherwise
    ///
    /// # Examples
    /// ```ignore
    /// while !parser.is_at_end() {
    ///     let decl = parser.declaration()?;
    ///     declarations.push(decl);
    /// }
    /// ```
    pub fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::EOF)
    }

    /// Looks ahead at the next token without consuming it.
    /// Useful for predictive parsing and conditional logic.
    ///
    /// # Returns
    /// Option containing reference to the next token-span pair, or None if at end
    ///
    /// # Examples
    /// ```ignore
    /// if let Some((next_token, _)) = parser.peek_token() {
    ///     if next_token == &Token::Equals {
    ///         // Handle assignment
    ///     }
    /// }
    /// ```
    pub fn peek_token(&self) -> Option<&(Token, Span)> {
        if self.position + 1 < self.tokens.len() {
            Some(&self.tokens[self.position + 1])
        } else {
            None
        }
    }

    /// Returns the actual current token from the tokens vector.
    /// Unlike `current_token()`, this returns the full token-span pair.
    ///
    /// # Returns
    /// Reference to the current (Token, Span) pair
    pub fn actual_token(&self) -> &(Token, Span) {
        &self.tokens[self.position]
    }

    /// Checks if the current token matches the given token.
    /// Does not consume the token - only performs a comparison.
    ///
    /// # Arguments
    /// * `token` - The token to compare against the current token
    ///
    /// # Returns
    /// `true` if current token matches, `false` otherwise
    ///
    /// # Examples
    /// ```ignore
    /// if parser.check(Token::Semicolon) {
    ///     // Current token is a semicolon
    /// }
    /// ```
    pub fn check(&self, token: Token) -> bool {
        self.current_token() == &token
    }

    /// Checks if the current token matches the given token and consumes it if true.
    /// This is the primary method for conditional token consumption.
    ///
    /// # Arguments
    /// * `token` - The token to match against
    ///
    /// # Returns
    /// `true` if token was matched and consumed, `false` otherwise
    ///
    /// # Examples
    /// ```ignore
    /// let is_mutable = parser.matches(Token::Mut);
    /// if is_mutable {
    ///     // 'mut' keyword was present and consumed
    /// }
    /// ```
    pub fn matches(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consumes the current token if it matches the expected token.
    /// If the token doesn't match, returns a ParserError with the given message.
    ///
    /// # Arguments
    /// * `token` - The expected token to consume
    /// * `error_msg` - Error message to return if token doesn't match
    ///
    /// # Returns
    /// `Ok(())` if token was consumed, `Err(ParserError)` otherwise
    ///
    /// # Examples
    /// ```ignore
    /// // Expect and consume a left parenthesis
    /// parser.consume(Token::LParen, "Expect '(' after function name")?;
    /// ```
    pub fn consume(&mut self, token: Token, error_msg: &str) -> Result<(), ParserError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(error_msg))
        }
    }

    /// Creates a ParserError with the given message and appropriate span.
    /// Automatically handles end-of-file cases with a specialized message.
    ///
    /// # Arguments
    /// * `message` - The error message describing what was expected
    ///
    /// # Returns
    /// A ParserError with message and source location span
    ///
    /// # Examples
    /// ```ignore
    /// let error = parser.error("Expected variable name");
    /// // Error will have message and point to current token location
    /// ```
    pub fn error(&self, message: &str) -> ParserError { // add span in the method so it always tells me where is it
        if self.is_at_end() {

            let token = self.previous_token().unwrap().clone();

            return ParserError {
                message: format!("{:?} is at end of file, try finish with ';'", token.0),
                span: token.1
            }
        }
        ParserError {
            message: message.to_string(),
            span: self.current_span().clone(),
        }
    }

    /// Recovers from a parsing error by synchronizing to the next statement boundary.
    /// Advances past tokens until it finds a likely statement starter or semicolon.
    /// This prevents cascading errors and allows parsing to continue.
    ///
    /// Synchronization points:
    /// - Semicolons (statement terminators)
    /// - Declaration keywords (fn, let, struct, enum)
    /// - Control flow keywords (if, while, for, return)
    ///
    /// # Examples
    /// ```ignore
    /// match parser.declaration() {
    ///     Ok(decl) => declarations.push(decl),
    ///     Err(err) => {
    ///         parser.errors.push(err);
    ///         parser.synchronize(); // Skip to next statement
    ///     }
    /// }
    /// ```
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

    /// Returns the previous token that was processed, if any.
    /// Useful for error reporting and span calculations.
    ///
    /// # Returns
    /// Option containing reference to the previous token-span pair
    ///
    /// # Examples
    /// ```ignore
    /// if let Some(prev_token) = parser.previous_token() {
    ///     let span = prev_token.1.merge(parser.current_span());
    /// }
    /// ```
    pub fn previous_token(&self) -> Option<&(Token, Span)> {
        if self.position > 0 {
            Some(&self.tokens[self.position - 1])
        } else {
            None
        }
    }

    /// Returns the next token without modifying the parser state.
    /// Similar to `peek_token()` but returns the token at current position.
    ///
    /// # Returns
    /// Reference to the current token in the tokens vector
    pub fn next_token(&self) -> Option<&(Token, Span)> {
        self.tokens.iter().nth(self.position)
    }

    /// Takes all accumulated parsing errors and returns them.
    /// This is typically called at the end of parsing to collect all errors.
    ///
    /// # Returns
    /// Vector of ParserError objects collected during parsing
    ///
    /// # Examples
    /// ```ignore
    /// let result = parser.parse();
    /// if let Err(errors) = result {
    ///     for error in errors {
    ///         eprintln!("Error: {}", error);
    ///     }
    /// }
    /// ```
    pub fn take_errors(&self) -> Vec<ParserError> {
        self.errors.clone()
    }

}