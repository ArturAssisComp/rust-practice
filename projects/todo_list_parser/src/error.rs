use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    //Malformed,
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

impl Error for ReadError {
    fn description(&self) -> &str {
        "Todo list read failed!"
    }
    fn cause(&self) -> Option<&dyn Error> {
        Some(&*self.child_error)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed parsing todo file")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Todo list parse failed!"
    }
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
