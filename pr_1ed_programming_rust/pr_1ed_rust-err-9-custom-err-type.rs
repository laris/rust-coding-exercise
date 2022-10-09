// json/src/error.rs

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line:usize,
    pub column: usize,
}

return Err(JsonError {
    mesage: "expected ']' at end of array".to_string(),
    line: current_line,
    column: current_column
});

use std;
use std::fmt;

// Errors should be printable.
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Errors should implement the std::error::Error trait
impl std::error::Error for JsonError {
    fn description(&self) -> &str {
        &self.message
    }
}
