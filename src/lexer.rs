use std::{collections::HashMap, vec};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    QuestionMark, Colon,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False, True, Fun, For,
    If, Null, Or, Print, Return, Super, This,
    Let, While,

    EOF
}

#[derive(Debug,Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    Identifier(String)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl Token {

    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number
        }
    }

    // not modify or delete
    pub fn to_string(self: &Self) -> String {
        format!("{}",self.lexeme)
    }

}

pub fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("true", TokenType::True),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("null", TokenType::Null),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("let", TokenType::Let),
        ("while", TokenType::While),
    ])
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<&'static str,TokenType>
}

impl Lexer {

    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords_hashmap()
        }
    }

    // principal scan
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        
        let mut errors: Vec<String> = vec![];

        // while source hasnt end scan token and push errors if they are
        // and update self.start
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        // push end file
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "End of File".to_string(),
            literal: None,
            line_number: self.line, 
        });

        // collect errors
        if errors.len() > 0 {
            let mut joined: String = String::new();
                for msg in &errors {
                    joined.push_str(msg);
                    joined.push_str("\n");
                }
            return Err(joined)
        }

        Ok(self.tokens.clone())

    }

    fn scan_token(&mut self) -> Result<(), String> {

        // consume char
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '?' => self.add_token(TokenType::QuestionMark),
            ':' => self.add_token(TokenType::Colon),
            '!' | '=' | '<' | '>' => {
            let token: TokenType = match (c, self.char_match('=')) {
                ('!', true) => TokenType::BangEqual,
                ('!', false) => TokenType::Bang,
                ('=', true) => TokenType::EqualEqual,
                ('=', false) => TokenType::Equal,
                ('<', true) => TokenType::LessEqual,
                ('<', false) => TokenType::Less,
                ('>', true) => TokenType::GreaterEqual,
                ('>', false) => TokenType::Greater,
                _ => return Err(format!("Unrecognized token: {}", c)),
            };
            self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {

                    // while comment exist, skip
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }

                } else if self.char_match( '*'){

                    let start_line: usize = self.current;

                    while !self.is_at_end() {
                        if self.peek() == '*' && self.peek_next() == '/' {
                            self.advance(); // Advance to '*'
                            self.advance(); // Advance to '/'
                            break;
                        } 
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }

                    if self.is_at_end() {
                        return Err(format!("Unterminated multi line comment in line {}",start_line));
                    }
                    
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string()?,
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            c => {
                if self.is_digit(c) {
                    self.number()?;
                }else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(format!("Unrecognized char: {} at line: {}", c, self.line)) 
                }
            }
        }
        Ok(())
    }

    //  ------------------------------------- UTILS -----------------------------------------------------------

    // if char at end return null, otherwise return char without advancing
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        self.source.chars().nth(self.current).unwrap()
    }

     // if char at end return null, otherwise return the next char without advancing
    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, ch: char) -> bool {
        let uch: u8 = ch as u8;
        return uch >= '0' as u8 && uch <= '9' as u8
    }

    fn is_alpha(&self, ch: char) -> bool {
        let uch: u8 = ch as u8;
        return (uch >= 'a' as u8 && uch <= 'z' as u8) || (uch >= 'A' as u8 && uch <= 'Z' as u8) || (ch == '_') || (ch == '-')
    }

    fn is_alpha_numeric(&self, ch: char) -> bool { self.is_alpha(ch) || self.is_digit(ch) }

    // ---------------------------------------------- LITERAL IDENTIFIERS ----------------------------------------------------------

    fn identifier(&mut self) {

        // while is a valid alpha numeric advance
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        
        // save lexeme
        let lexeme: &str = &self.source[self.start..self.current];
    
        if let Some(&token_type) = self.keywords.get(lexeme) {
            self.add_token(token_type);
        } else {
            self.add_token_lit(TokenType::Identifier,Some(LiteralValue::Identifier(lexeme.to_string())));
        }
    }

    fn number(&mut self) -> Result<(), String> {
        // while is still a number advance
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // if its a float still advance and continue reading
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let substring: &str = &self.source[self.start..self.current];
        let value: Result<f64, std::num::ParseFloatError> = substring.parse::<f64>();

        match value {
            Ok(value) =>  self.add_token_lit(TokenType::Number,Some(LiteralValue::FloatValue(value))),
            Err(_) => return Err(format!("Could not parse number {} in line {}", substring, self.line)),
        }

        Ok(())
    }

    // read string 
    fn string(&mut self) -> Result<(), String> {

        // while string hasnt ended keep advancing
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    
        // if string is unterminated return error
        if self.is_at_end() {
            return Err(format!("Unterminated string at line: {}", self.line));
        }
    
        // consume '"'
        self.advance();

        // value is the string without the ""
        let value: &str = &self.source[self.start + 1..self.current - 1];

        // save string with literal
        self.add_token_lit(TokenType::String, Some(LiteralValue::StringValue(value.to_string())));
    
        Ok(())
    }

    // ------------------------------------- LEXER FUNCTIONS --------------------------------------------------
    
    // if the char is what expected consume the char
    fn char_match(&mut self, expected: char) -> bool {
        if !self.is_at_end() && self.source.chars().nth(self.current).unwrap() == expected {
            self.advance();
            return true;
        }
        false
    }

    // read, consume and return char
    fn advance(&mut self) -> char {
        let c: char = self.source.chars().nth(self.current).unwrap();
        self.current +=1;
        
        c as char
    }

    // add token to self.tokens with token_type and without literal
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_lit(token_type, None)
    }

    // add token with token_type lexeme, literal and line
    fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {

        let text: String = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line,
        });
    }

}

// ---------------------------------------------------- TESTS ----------------------------------------------------

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source: &str = "(( ))";
        let mut lexer: Lexer = Lexer::new(source);

        let _ = lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(),5);
        assert_eq!(lexer.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(lexer.tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(lexer.tokens[2].token_type, TokenType::RightParen);
        assert_eq!(lexer.tokens[3].token_type, TokenType::RightParen);
        assert_eq!(lexer.tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source: &str = "! != = == > >= < <= //";
        let mut lexer: Lexer = Lexer::new(source);

        let _ = lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(),9);
        assert_eq!(lexer.tokens[0].token_type, TokenType::Bang);
        assert_eq!(lexer.tokens[1].token_type, TokenType::BangEqual);
        assert_eq!(lexer.tokens[2].token_type, TokenType::Equal);
        assert_eq!(lexer.tokens[3].token_type, TokenType::EqualEqual);
        assert_eq!(lexer.tokens[4].token_type, TokenType::Greater);
        assert_eq!(lexer.tokens[5].token_type, TokenType::GreaterEqual);
        assert_eq!(lexer.tokens[6].token_type, TokenType::Less);
        assert_eq!(lexer.tokens[7].token_type, TokenType::LessEqual);
        assert_eq!(lexer.tokens[8].token_type, TokenType::EOF);
    }

    #[test]
    fn handle_multi_line_comment() {
        let source: &str = "/* this\n is\n a\n multi\n line\n comment\n */";
        let mut lexer: Lexer = Lexer::new(source);

        let _ = lexer.scan_tokens();

        assert_eq!(lexer.tokens.len(),1);
        assert_eq!(lexer.line, 7);
        assert_eq!(lexer.tokens[0].token_type, TokenType::EOF)
    }

    #[test]
    fn handle_multi_line_comment_unterminated() {
        let source: &str = "/* this\n is\n a\n multi\n line\n comment\n";
        let mut lexer: Lexer = Lexer::new(source);

        let result: Result<Vec<Token>, String> = lexer.scan_tokens();
        
        match result {
            Err(msg) => assert!(msg.contains("Unterminated multi line comment in line 2")),
            _ => panic!("Should have failed with an unterminated multi line comment error"),
        }
    }

    #[test]
    fn handle_string_lit() {
        let source: &str = r#""this is a string""#;
        let mut lexer: Lexer = Lexer::new(source);

        let _ = lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(),2);
        assert_eq!(lexer.tokens[0].token_type, TokenType::String);
        
        match lexer.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => 
                assert_eq!(val, "this is a string"),
                _ => panic!("Incorrect literal type")
        };
    }

    #[test]
    fn handle_string_lit_unterminated() {
        let source: &str = r#""this is a unterminated string"#;
        let mut lexer: Lexer = Lexer::new(source);
        let result: Result<Vec<Token>, String> = lexer.scan_tokens();
    
        match result {
            Err(msg) => assert!(msg.contains("Unterminated string")),
            _ => panic!("Should have failed with an unterminated string error"),
        }
    }

    #[test]
    fn handle_string_lit_multiline() {
        let source: &str = "\"this is a\n multi line string\"";
        let mut lexer: Lexer = Lexer::new(source);

        let _ = lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(),2);
        assert_eq!(lexer.tokens[0].token_type, TokenType::String);

        match lexer.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => assert_eq!(val,"this is a\n multi line string"),
            _ => panic!("Incorrect literal type")
        }
    }

    #[test]
    fn handle_literals() {
        let source: &str = "123.123\n123.0\n5";
        let mut lexer: Lexer = Lexer::new(source);

        _ = lexer.scan_tokens().unwrap();

        assert_eq!(lexer.tokens.len(),4);

        for i in 0..3 {
            assert_eq!(lexer.tokens[i].token_type, TokenType::Number);
        }

        match lexer.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::FloatValue(val) => assert_eq!(*val, 123.123),
            _ => panic!("Incorect number")
        }
        match lexer.tokens[1].literal.as_ref().unwrap() {
            LiteralValue::FloatValue(val) => assert_eq!(*val, 123.0),
            _ => panic!("Incorect number")
        }
        match lexer.tokens[2].literal.as_ref().unwrap() {
            LiteralValue::FloatValue(val) => assert_eq!(*val, 5.0),
            _ => panic!("Incorect number")
        }
    }
    
    #[test]
    fn handle_identifier() {
        let source: &str = "t = 1;";
        let mut lexer: Lexer = Lexer::new(source);

        _ = lexer.scan_tokens().unwrap();
        assert_eq!(lexer.tokens.len(), 5);
        
        assert_eq!(lexer.tokens[0].token_type, TokenType::Identifier);
        assert_eq!(lexer.tokens[1].token_type, TokenType::Equal);
        assert_eq!(lexer.tokens[2].token_type, TokenType::Number);
        assert_eq!(lexer.tokens[3].token_type, TokenType::Semicolon);
        assert_eq!(lexer.tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn get_keywords() {
        let source: &str = "let x = 1; \n while ! true { print 10 };";
        let mut lexer: Lexer = Lexer::new(source);
    
        let _ = lexer.scan_tokens().unwrap();
    
        // Assert the number of tokens
        assert_eq!(lexer.tokens.len(), 14);

        // Check the token types
        assert_eq!(lexer.tokens[0].token_type, TokenType::Let);         // "let"
        assert_eq!(lexer.tokens[1].token_type, TokenType::Identifier);  // "x"
        assert_eq!(lexer.tokens[2].token_type, TokenType::Equal);       // "="
        assert_eq!(lexer.tokens[3].token_type, TokenType::Number);      // "1"
        assert_eq!(lexer.tokens[4].token_type, TokenType::Semicolon);   // ";"
        assert_eq!(lexer.tokens[5].token_type, TokenType::While);       // "while"
        assert_eq!(lexer.tokens[6].token_type, TokenType::Bang);       // "!"
        assert_eq!(lexer.tokens[7].token_type, TokenType::True);        // "true"
        assert_eq!(lexer.tokens[8].token_type, TokenType::LeftBrace);   // "{"
        assert_eq!(lexer.tokens[9].token_type, TokenType::Print);       // "print"
        assert_eq!(lexer.tokens[10].token_type, TokenType::Number);      // "10"
        assert_eq!(lexer.tokens[11].token_type, TokenType::RightBrace); // "}"
        assert_eq!(lexer.tokens[12].token_type, TokenType::Semicolon); // ";"
        assert_eq!(lexer.tokens[13].token_type, TokenType::EOF);
    }

}