use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    IntLiteral(i64),
    UintLiteral(u64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    CharLiteral(char),
    Identifier(String),
    
    // Keywords
    Let,
    If,
    Else,
    While,
    For,
    Fn,
    Enum,
    Struct,
    Match,
    Mut,
    Return,
    
    // Symbols
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    BangEquals,
    Equals,
    DoubleEquals,
    Modulo,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Arrow, // =>
    DoublePoint, // ..
    
    // Special
    EOF,
    Illegal,
}

impl Token {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Literals - return descriptive type names
            Token::IntLiteral(_) => "integer literal",
            Token::UintLiteral(_) => "unsigned integer literal",
            Token::FloatLiteral(_) => "float literal",
            Token::BoolLiteral(_) => "boolean literal",
            Token::StringLiteral(_) => "string literal",
            Token::CharLiteral(_) => "character literal",
            Token::Identifier(_) => "identifier",
            
            // Keywords - return the keyword itself
            Token::Let => "let",
            Token::If => "if",
            Token::Else => "else",
            Token::While => "while",
            Token::For => "for",
            Token::Fn => "fn",
            Token::Enum => "enum",
            Token::Struct => "struct",
            Token::Match => "match",
            Token::Mut => "mut",
            Token::Return => "return",
            
            // Symbols - return the symbol
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Bang => "!",
            Token::Less => "<",
            Token::LessEqual => "<=",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            Token::BangEquals => "!=",
            Token::Equals => "=",
            Token::DoubleEquals => "==",
            Token::Modulo => "%",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::LBracket => "[",
            Token::RBracket => "]",
            Token::Colon => ":",
            Token::Semicolon => ";",
            Token::Comma => ",",
            Token::Dot => ".",
            Token::Arrow => "=>",
            Token::DoublePoint => "..",
            
            // Special
            Token::EOF => "end of file",
            Token::Illegal => "illegal token",
        }
    }
}