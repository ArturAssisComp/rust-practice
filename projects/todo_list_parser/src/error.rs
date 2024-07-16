use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    Malformed,
    Empty,
}

#[derive(Debug)]
pub struct ReadError {
    pub child_error: Box<dyn Error>,
}

impl Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed reading todo file")
    }
}
