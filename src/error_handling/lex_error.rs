use crate::error_handling::{report_error_str, Error};

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: u32,
}

impl LexError {
    pub fn new(message: String, line: u32) -> LexError {
        LexError { message, line }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", report_error_str(self.line, self.message.clone()))
    }
}

impl Error for LexError {
    fn line(&self) -> u32 {
        self.line
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}
