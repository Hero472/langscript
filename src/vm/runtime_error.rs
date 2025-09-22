use crate::{error::{RuntimeError, RuntimeErrorType}, frontend::lexer::span::Span};

impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            line: None,
            column: None,
            end_line: None,
            end_column: None,
            file: None,
            stack_trace: Vec::new(),
            error_type: RuntimeErrorType::Custom("RuntimeError".to_string()),
        }
    }

    // Builder pattern methods
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.line = Some(span.start_line);
        self.column = Some(span.start_column);
        self.end_line = Some(span.end_line);
        self.end_column = Some(span.end_column);
        
        // if let Some(file_id) = span.file_id {
        //     self.file = Some(self.resolve_file_id(file_id));
        // }
        
        self
    }

    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }

    pub fn with_type(mut self, error_type: RuntimeErrorType) -> Self {
        self.error_type = error_type;
        self
    }

    pub fn add_stack_frame(mut self, frame: String) -> Self {
        self.stack_trace.push(frame);
        self
    }

    pub fn extend_stack_trace(mut self, frames: Vec<String>) -> Self {
        self.stack_trace.extend(frames);
        self
    }

    // Getters
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn line(&self) -> Option<usize> {
        self.line
    }

    pub fn column(&self) -> Option<usize> {
        self.column
    }

    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    pub fn stack_trace(&self) -> &[String] {
        &self.stack_trace
    }

    pub fn error_type(&self) -> &RuntimeErrorType {
        &self.error_type
    }

    // Convenience constructors for common error types
    pub fn type_error(message: String) -> Self {
        Self::new(message).with_type(RuntimeErrorType::TypeError)
    }

    pub fn value_error(message: String) -> Self {
        Self::new(message).with_type(RuntimeErrorType::ValueError)
    }

    pub fn division_by_zero() -> Self {
        Self::new("Division by zero".to_string())
            .with_type(RuntimeErrorType::DivisionByZero)
    }

    pub fn overflow() -> Self {
        Self::new("Arithmetic overflow".to_string())
            .with_type(RuntimeErrorType::Overflow)
    }

    pub fn underflow() -> Self {
        Self::new("Arithmetic underflow".to_string())
            .with_type(RuntimeErrorType::Underflow)
    }

    pub fn undefined_variable(name: &str) -> Self {
        Self::new(format!("Undefined variable: '{}'", name))
            .with_type(RuntimeErrorType::UndefinedVariable)
    }

    pub fn index_out_of_bounds(index: usize, length: usize) -> Self {
        Self::new(format!("Index {} out of bounds for length {}", index, length))
            .with_type(RuntimeErrorType::IndexOutOfBounds)
    }

    pub fn invalid_operation(message: String) -> Self {
        Self::new(message)
            .with_type(RuntimeErrorType::InvalidOperation)
    }

    // Formatting
    pub fn to_string_with_location(&self) -> String {
        let mut result = format!("{}: {}", self.error_type, self.message);
        
        if let Some(file) = &self.file {
            result.push_str(&format!("\n  at {}:", file));
            if let Some(line) = self.line {
                result.push_str(&format!("{}", line));
                if let Some(column) = self.column {
                    result.push_str(&format!(":{}", column));
                }
            }
        }
        
        if !self.stack_trace.is_empty() {
            result.push_str("\nStack trace:");
            for (i, frame) in self.stack_trace.iter().enumerate() {
                result.push_str(&format!("\n  {}: {}", i, frame));
            }
        }
        
        result
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string_with_location())
    }
}

impl std::fmt::Display for RuntimeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeErrorType::TypeError => write!(f, "TypeError"),
            RuntimeErrorType::ValueError => write!(f, "ValueError"),
            RuntimeErrorType::DivisionByZero => write!(f, "DivisionByZeroError"),
            RuntimeErrorType::Overflow => write!(f, "OverflowError"),
            RuntimeErrorType::Underflow => write!(f, "UnderflowError"),
            RuntimeErrorType::UndefinedVariable => write!(f, "UndefinedVariableError"),
            RuntimeErrorType::IndexOutOfBounds => write!(f, "IndexOutOfBoundsError"),
            RuntimeErrorType::InvalidOperation => write!(f, "InvalidOperationError"),
            RuntimeErrorType::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for RuntimeError {}

pub fn wrap_error<E: Into<String>>(error: E, span: Span) -> RuntimeError {
        RuntimeError::new(error.into()).with_span(span)
    }