#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub file_id: Option<usize>, // Optional: for multiple files
}

impl Span {
    pub fn new(start_line: usize, start_column: usize, end_line: usize, end_column: usize) -> Self {
        Self {
            start_line,
            start_column,
            end_line,
            end_column,
            file_id: None,
        }
    }
    
    pub fn with_file(mut self, file_id: usize) -> Self {
        self.file_id = Some(file_id);
        self
    }
}