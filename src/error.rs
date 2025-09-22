use crate::frontend::lexer::span::Span;

#[derive(Debug, Clone)]
pub enum CompilationError {
    ParserError(ParserError),
    TypeError(TypeError),
    //RuntimeError(RuntimeError)
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub span: Span,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub span: Span,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub end_line: Option<usize>,
    pub end_column: Option<usize>,
    pub file: Option<String>,
    pub stack_trace: Vec<String>,
    pub error_type: RuntimeErrorType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeErrorType {
    TypeError,
    ValueError,
    DivisionByZero,
    Overflow,
    Underflow,
    UndefinedVariable,
    IndexOutOfBounds,
    InvalidOperation,
    Custom(String),
}

// Implement constructors
impl TypeError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl ParserError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

// Implement Display for all error types
impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationError::ParserError(e) => write!(f, "Parser error: {}", e),
            CompilationError::TypeError(e) => write!(f, "Type error: {}", e),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {:?}: {}", self.span, self.message)
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {:?}: {}", self.span, self.message)
    }
}

impl std::error::Error for CompilationError {}
impl std::error::Error for ParserError {}
impl std::error::Error for TypeError {}

// Conversion implementations for ?
impl From<ParserError> for CompilationError {
    fn from(error: ParserError) -> Self {
        CompilationError::ParserError(error)
    }
}

impl From<TypeError> for CompilationError {
    fn from(error: TypeError) -> Self {
        CompilationError::TypeError(error)
    }
}