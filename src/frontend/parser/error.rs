use crate::frontend::lexer::span::Span;



#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
}

impl ParserError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}