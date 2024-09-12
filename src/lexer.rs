#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword(String),
    Identifier(String),
    Operator(String),
    Number(f64),
    ParenLeft,
    ParenRight,
    EOF,
}

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {

    pub fn new(input: String) -> Self {
        let mut lexer: Lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    pub fn advance(&mut self) {
        self.current_char = if self.position < self.input.len() {
            Some(self.input.chars().nth(self.position).unwrap())
        } else {
            None
        };
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> TokenType {
        let mut number: String = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) || c == '.' {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }
        TokenType::Number(number.parse::<f64>().unwrap())
    }

    fn read_identifier(&mut self) -> TokenType {
        let mut ident: String = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "if" | "else" | "let" => TokenType::Keyword(ident),
            _ => TokenType::Identifier(ident),
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        if let Some(c) = self.current_char {
            match c {
                '(' => {
                    self.advance();
                    return TokenType::ParenLeft;
                }
                ')' => {
                    self.advance();
                    return TokenType::ParenRight;
                }
                '0'..='9' => return self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(),
                _ => {
                    self.advance();
                    return TokenType::Operator(c.to_string());
                }
            }
        }

        TokenType::EOF
    }

}