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

    pub fn merge(&self, other: &Span) -> Span {

        if self.file_id != other.file_id {

            let merged_file_id = if self.file_id.is_some() { 
                self.file_id 
            } else { 
                other.file_id 
            };
            
            return Span {
                start_line: self.start_line.min(other.start_line),
                start_column: if self.start_line < other.start_line {
                    self.start_column
                } else if self.start_line > other.start_line {
                    other.start_column
                } else {
                    self.start_column.min(other.start_column)
                },
                end_line: self.end_line.max(other.end_line),
                end_column: if self.end_line > other.end_line {
                    self.end_column
                } else if self.end_line < other.end_line {
                    other.end_column
                } else {
                    self.end_column.max(other.end_column)
                },
                file_id: merged_file_id,
            };
        }
        
        Span {
            start_line: self.start_line.min(other.start_line),
            start_column: if self.start_line < other.start_line {
                self.start_column
            } else if self.start_line > other.start_line {
                other.start_column
            } else {
                self.start_column.min(other.start_column)
            },
            end_line: self.end_line.max(other.end_line),
            end_column: if self.end_line > other.end_line {
                self.end_column
            } else if self.end_line < other.end_line {
                other.end_column
            } else {
                self.end_column.max(other.end_column)
            },
            file_id: self.file_id,
        }
    }
}