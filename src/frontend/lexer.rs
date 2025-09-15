use super::tokens::{Token, Span};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1
        }
    }

    pub fn tokenize(&mut self) -> Vec<(Token, Span)> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let start = self.position;
            let start_line = self.line;
            let start_col = self.column;

            let c = self.current_char();

            if let Some(c) = c {
                let token = match c {
                    // Whitespace
                    ' ' | '\t' | '\r' => {
                        self.advance();
                        continue;
                    }
                    '\n' => {
                        self.advance();
                        continue;
                    }
                    
                    // Single character symbols
                    '(' => {
                        self.advance();
                        Token::LParen
                    }
                    ')' => {
                        self.advance();
                        Token::RParen
                    }
                    '{' => {
                        self.advance();
                        Token::LBrace
                    }
                    '}' => {
                        self.advance();
                        Token::RBrace
                    }
                    '[' => {
                        self.advance();
                        Token::LBracket
                    }
                    ']' => {
                        self.advance();
                        Token::RBracket
                    }
                    ':' => {
                        self.advance();
                        Token::Colon
                    }
                    ';' => {
                        self.advance();
                        Token::Semicolon
                    }
                    ',' => {
                        self.advance();
                        Token::Comma
                    }
                    '.' => {
                        // Check for double dot (..)
                        if self.peek_char() == Some('.') {
                            self.advance(); // consume first dot
                            self.advance(); // consume second dot
                            Token::DoublePoint
                        } else {
                            self.advance();
                            Token::Dot
                        }
                    }
                    '+' => {
                        self.advance();
                        Token::Plus
                    }
                    '-' => {
                        // Check for arrow (=>)
                        if self.peek_char() == Some('>') {
                            self.advance(); // consume '-'
                            self.advance(); // consume '>'
                            Token::Arrow
                        } else {
                            self.advance();
                            Token::Minus
                        }
                    }
                    '*' => {
                        self.advance();
                        Token::Star
                    }
                    // Comments
                    '/' => {
                        // Check for single-line comment (//)
                        if self.peek_char() == Some('/') {
                            self.consume_single_line_comment();
                            continue;
                        }
                        // Check for multi-line comment (/*)
                        else if self.peek_char() == Some('*') {
                            if let Err(_) = self.consume_multi_line_comment() {
                                Token::Illegal // Unclosed multi-line comment
                            } else {
                                continue; // Comment consumed successfully
                            }
                        }
                        // Regular slash operator
                        else {
                            self.advance();
                            Token::Slash
                        }
                    }
                    '=' => {
                        // Check for double equals (==)
                        if self.peek_char() == Some('=') {
                            self.advance(); // consume first '='
                            self.advance(); // consume second '='
                            Token::DoubleEquals
                        } else {
                            self.advance();
                            Token::Equals
                        }
                    },
                    '<' => {
                        if self.peek_char() == Some('=') {
                            self.advance(); // consume first '<'
                            self.advance(); // consume second '='
                            Token::LessEqual
                        } else {
                            self.advance();
                            Token::Less
                        }
                    },
                    '>' => {
                        if self.peek_char() == Some('=') {
                            self.advance(); // consume first '>'
                            self.advance(); // consume second '='
                            Token::GreaterEqual
                        } else {
                            self.advance();
                            Token::Greater
                        }
                    }
                    
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let ident = self.consume_identifier();
                        self.match_keyword(&ident)
                    }
                    
                    // Numbers
                    '0'..='9' => {
                        self.consume_number()
                    }
                    
                    // String literals
                    '"' => {
                        self.consume_string()
                    }
                    
                    // Unknown character
                    _ => {
                        self.advance();
                        Token::Illegal
                    }
                };

                let span = Span {
                    start,
                    end: self.position,
                    line: start_line,
                    column: start_col,
                };

                tokens.push((token, span));
            } else {
                break;
            }
        }

        // Add EOF token
        let eof_span = Span {
            start: self.position,
            end: self.position,
            line: self.line,
            column: self.column,
        };
        tokens.push((Token::EOF, eof_span));

        tokens
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()   
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().nth(1)
    }

    fn consume_identifier(&mut self) -> String {
        let mut ident = String::new();
        
        while let Some(c) = self.current_char() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        ident
    }

    fn match_keyword(&self, ident: &str) -> Token {
        match ident {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "fn" => Token::Fn,
            "enum" => Token::Enum,
            "struct" => Token::Struct,
            "match" => Token::Match,
            "true" => Token::BoolLiteral(true),
            "false" => Token::BoolLiteral(false),
            "mut" => Token::Mut,
            _ => Token::Identifier(ident.to_string()),
        }
    }

    fn consume_number(&mut self) -> Token {
        let start = self.position;
        let mut has_dot = false;
        
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !has_dot {
                // Check if next character is a digit to avoid parsing "1." as float
                if let Some(next) = self.peek_char() {
                    if next.is_ascii_digit() {
                        has_dot = true;
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        let num_str = &self.input[start..self.position];
        
        if has_dot {
            Token::FloatLiteral(num_str.parse().unwrap_or(0.0))
        } else {
            Token::IntLiteral(num_str.parse().unwrap_or(0))
        }
    }

    fn consume_string(&mut self) -> Token {
        self.advance(); // consume opening quote
        
        let mut string_content = String::new();
        let mut escaped = false;
        
        while let Some(c) = self.current_char() {
            if escaped {
                match c {
                    'n' => string_content.push('\n'),
                    't' => string_content.push('\t'),
                    'r' => string_content.push('\r'),
                    '"' => string_content.push('"'),
                    '\\' => string_content.push('\\'),
                    _ => string_content.push(c), // just include the character as-is
                }
                escaped = false;
                self.advance();
            } else if c == '\\' {
                escaped = true;
                self.advance();
            } else if c == '"' {
                self.advance(); // consume closing quote
                break;
            } else {
                string_content.push(c);
                self.advance();
            }
        }
        
        Token::StringLiteral(string_content)
    }

    fn consume_single_line_comment(&mut self) {
        // Consume the two slashes
        self.advance(); // first '/'
        self.advance(); // second '/'
        
        // Consume everything until end of line or end of input
        while let Some(c) = self.current_char() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn consume_multi_line_comment(&mut self) -> Result<(), ()> {
        // Consume the opening /*
        self.advance(); // consume '/'
        self.advance(); // consume '*'
        
        let mut depth = 1; // track nested comments
        
        while depth > 0 {
            match (self.current_char(), self.peek_char()) {
                (Some('/'), Some('*')) => {
                    // Nested comment opening /*
                    self.advance(); // consume '/'
                    self.advance(); // consume '*'
                    depth += 1;
                }
                (Some('*'), Some('/')) => {
                    // Comment closing */
                    self.advance(); // consume '*'
                    self.advance(); // consume '/'
                    depth -= 1;
                }
                (Some('\n'), _) => {
                    self.advance(); // consume newline, line/column updated automatically
                }
                (Some(_), _) => {
                    self.advance(); // consume any other character
                }
                (None, _) => {
                    // End of input before comment closed
                    return Err(());
                }
            }
        }
        
        Ok(())
    }
    
    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            self.position += c.len_utf8();

            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }

}