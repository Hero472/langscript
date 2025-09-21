#[derive(Debug, Clone)]
pub struct RuntimeError {
    message: String,
    line: Option<u32>,
    column: Option<u32>,
    file: Option<String>,
    stack_trace: Vec<String>,
    error_type: RuntimeErrorType,
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

impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            line: None,
            column: None,
            file: None,
            stack_trace: Vec::new(),
            error_type: RuntimeErrorType::Custom("RuntimeError".to_string()),
        }
    }
}