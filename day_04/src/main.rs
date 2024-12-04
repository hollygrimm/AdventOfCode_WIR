//! Day 4: Pattern Finder
//!
//! This program searches for patterns in a 2D character grid:
//! 1. Finds instances of "XMAS" in any direction (including backwards)
//! 2. Finds instances of "MAS" arranged in X patterns
//!
//! # Usage
//!
//! ```bash
//! cargo run -- path/to/input/file
//! ```
use std::error::Error;

// Internal imports
mod calculations;
mod errors;
mod file_io;

use calculations::{count_instances, count_x_instances};
use errors::AppError;
use file_io::read_file;

/// Main function that processes the input file and reports pattern matches.
///
/// # Arguments
///
/// Expects one command-line argument: the path to the input file
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Success or an error if the file cannot be processed
fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Day 4!");
    let path = std::env::args()
        .nth(1)
        .ok_or(AppError::ArgError("No input file provided"))?;
    let input = read_file(&path)?;

    let num_xmas_instances = count_instances(&input, "XMAS")?;
    println!("Instances of XMAS: {}", num_xmas_instances);

    let num_x_mas_instances = count_x_instances(&input, "MAS")?;
    println!("Instances of MAS in X shape: {}", num_x_mas_instances);

    Ok(())
}
