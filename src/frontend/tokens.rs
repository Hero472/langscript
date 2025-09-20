#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    IntLiteral(i64),
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
    BitwiseNot,
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

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}