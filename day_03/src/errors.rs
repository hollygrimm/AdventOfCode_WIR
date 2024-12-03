use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    ArgError(&'static str),
    ParseError(std::num::ParseIntError),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseError(error)
    }
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ArgError(msg) => write!(f, "Argument error: {}", msg),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}
