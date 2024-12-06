use std::error::Error;
use std::fmt;
use std::io;

/// Custom error types for the application
#[derive(Debug)]
pub enum AppError {
    /// Represents I/O operation failures
    IoError(io::Error),
    /// Represents missing or invalid command line arguments
    ArgError(&'static str),
    /// Represents failure to create an ndarray Array2 from input data
    Array2CreationError,
    /// Represents failure to find a starting position in the grid
    NoStartPosition,
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

impl From<ndarray::ShapeError> for AppError {
    fn from(_: ndarray::ShapeError) -> Self {
        Self::Array2CreationError
    }
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ArgError(msg) => write!(f, "Argument error: {}", msg),
            Self::Array2CreationError => write!(f, "Failed to create Array2 from input data"),
            Self::NoStartPosition => write!(f, "No starting position found in grid"),
        }
    }
}
