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

    /// Creates a span that represents a single point
    pub fn point(line: usize, column: usize, file_id: Option<usize>) -> Self {
        Self {
            start_line: line,
            start_column: column,
            end_line: line,
            end_column: column,
            file_id,
        }
    }

    pub fn to_user_friendly(&self) -> String {
        if let Some(file_id) = self.file_id {
            if self.start_line == self.end_line {
                if self.start_column == self.end_column {
                    format!("in file {} at line {}", file_id, self.start_line)
                } else {
                    format!(
                        "in file {} at line {}, columns {}-{}",
                        file_id, self.start_line, self.start_column, self.end_column
                    )
                }
            } else {
                format!(
                    "in file {} from line {} to line {}",
                    file_id, self.start_line, self.end_line
                )
            }
        } else {
            if self.start_line == self.end_line {
                if self.start_column == self.end_column {
                    format!("at line {}", self.start_line)
                } else {
                    format!(
                        "at line {}, columns {}-{}",
                        self.start_line, self.start_column, self.end_column
                    )
                }
            } else {
                format!("from line {} to line {}", self.start_line, self.end_line)
            }
        }
    }

}