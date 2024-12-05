//! Error types for the application.
//! 
//! This module defines the custom error types used throughout the application,
//! including IO errors, argument parsing errors, and number parsing errors.

use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    /// Represents errors that occur during file operations
    IoError(io::Error),
    /// Represents errors in command line arguments
    ArgError(&'static str),
    /// Represents errors in parsing string to integers
    ParseError(std::num::ParseIntError),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<&'static str> for AppError {
    fn from(error: &'static str) -> Self {
        Self::ArgError(error)
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
