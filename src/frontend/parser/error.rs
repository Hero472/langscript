use std::{error::Error, fmt};

use crate::frontend::lexer::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
    pub error_type: ParserErrorType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorType {
    SyntaxError,
    UnexpectedToken,
    ExpectedToken,
    InvalidExpression,
    UnterminatedString,
    UnterminatedComment,
    InvalidNumber,
    Custom,
}

impl ParserError {
    pub fn new(message: String, span: Span) -> Self {
        Self {
            message,
            span,
            error_type: ParserErrorType::Custom,
        }
    }

    pub fn with_type(message: String, span: Span, error_type: ParserErrorType) -> Self {
        Self {
            message,
            span,
            error_type,
        }
    }

    // Convenience constructors for common error types
    pub fn syntax_error(message: String, span: Span) -> Self {
        Self::with_type(message, span, ParserErrorType::SyntaxError)
    }

    pub fn unexpected_token(expected: &str, found: &str, span: Span) -> Self {
        Self::with_type(
            format!("Expected '{}', but found '{}'", expected, found),
            span,
            ParserErrorType::UnexpectedToken,
        )
    }

    pub fn expected_token(expected: &str, span: Span) -> Self {
        Self::with_type(
            format!("Expected '{}'", expected),
            span,
            ParserErrorType::ExpectedToken,
        )
    }

    pub fn invalid_expression(message: String, span: Span) -> Self {
        Self::with_type(message, span, ParserErrorType::InvalidExpression)
    }

    pub fn unterminated_string(span: Span) -> Self {
        Self::with_type(
            "Unterminated string literal".to_string(),
            span,
            ParserErrorType::UnterminatedString,
        )
    }

    pub fn invalid_number(span: Span) -> Self {
        Self::with_type(
            "Invalid numeric literal".to_string(),
            span,
            ParserErrorType::InvalidNumber,
        )
    }

    // Utility methods
    pub fn is_recoverable(&self) -> bool {
        match self.error_type {
            ParserErrorType::SyntaxError
            | ParserErrorType::UnexpectedToken
            | ParserErrorType::ExpectedToken => true,
            ParserErrorType::UnterminatedString
            | ParserErrorType::UnterminatedComment
            | ParserErrorType::InvalidNumber => false,
            ParserErrorType::InvalidExpression | ParserErrorType::Custom => true,
        }
    }

    pub fn with_context(self, context: &str) -> Self {
        Self {
            message: format!("{}: {}", context, self.message),
            ..self
        }
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser error at {}: {}", self.span.to_user_friendly(), self.message)
    }
}

// Convenience conversion traits
impl From<(String, Span)> for ParserError {
    fn from((message, span): (String, Span)) -> Self {
        Self::new(message, span)
    }
}

impl From<(&str, Span)> for ParserError {
    fn from((message, span): (&str, Span)) -> Self {
        Self::new(message.to_string(), span)
    }
}

// Result type alias for convenience
pub type ParserResult<T> = Result<T, ParserError>;

// Extension trait for Result for easier error creation
pub trait ParserResultExt<T> {
    fn map_parser_error<F>(self, f: F) -> ParserResult<T>
    where
        F: FnOnce() -> ParserError;
}

impl<T, E> ParserResultExt<T> for Result<T, E> {
    fn map_parser_error<F>(self, f: F) -> ParserResult<T>
    where
        F: FnOnce() -> ParserError,
    {
        self.map_err(|_| f())
    }
}

// Macro for creating parser errors easily
#[macro_export]
macro_rules! parser_error {
    ($span:expr, $($arg:tt)*) => {
        ParserError::new(format!($($arg)*), $span)
    };
}

#[macro_export]
macro_rules! syntax_error {
    ($span:expr, $($arg:tt)*) => {
        ParserError::syntax_error(format!($($arg)*), $span)
    };
}