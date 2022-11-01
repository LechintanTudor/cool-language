use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct LexicalError {
    message: String,
    line: usize,
}

impl LexicalError {
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
