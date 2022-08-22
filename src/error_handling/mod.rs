mod lex_error;
pub use lex_error::LexError;

// create custom error trait for all error types to implement
pub trait Error {
    fn line(&self) -> u32;
    fn message(&self) -> String;
}

impl std::fmt::Display for Box<dyn Error> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line(), self.message())
    }
}

impl std::fmt::Debug for Box<dyn Error> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("line: {}", &self.line())
            .field("message: {}", &self.message())
            .finish()
    }
}

fn report_error_str(line: u32, message: String) -> String {
    format!("[Line {line}] Error: {message}\n")
}

#[allow(dead_code)]
fn report_error(line: u32, message: String) {
    print!("{}", report_error_str(line, message));
}

#[derive(Debug)]
pub struct CLArgsError {
    pub line: u32,
    pub message: String,
}

impl CLArgsError {
    pub fn new(line: u32, message: String) -> CLArgsError {
        CLArgsError { line, message }
    }
}

impl std::fmt::Display for CLArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", report_error_str(self.line, self.message.clone()))
    }
}

impl std::error::Error for CLArgsError {}

impl Error for CLArgsError {
    fn line(&self) -> u32 {
        self.line
    }
    fn message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Debug)]
pub struct IOError {
    pub line: u32,
    pub message: String,
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", report_error_str(self.line, self.message.clone()))
    }
}

impl IOError {
    pub fn new(line: u32, err: std::io::Error) -> IOError {
        IOError {
            line,
            message: err.to_string(),
        }
    }
}

impl Error for IOError {
    fn line(&self) -> u32 {
        self.line
    }
    fn message(&self) -> String {
        self.message.clone()
    }
}
