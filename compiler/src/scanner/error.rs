use std::error::Error;
use std::fmt;

/// Error returned by the scanner when it cannot categorize a token.
#[derive(Clone, Debug)]
pub struct LexicalError {
    /// Error message.
    message: String,
    /// Line at which the error occurred.
    line: usize,
}

impl LexicalError {
    /// Creates a new error.
    pub fn new<S>(message: S, line: usize) -> Self
    where
        S: Into<String>,
    {
        Self { message: message.into(), line }
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error on line {}: {}", self.line, self.message)
    }
}

impl Error for LexicalError {}
