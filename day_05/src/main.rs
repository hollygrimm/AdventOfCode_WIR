//! Main entry point for the sequence processing application.
//! 
//! This application reads sequences and ordering rules from a file,
//! processes them according to the rules, and outputs a total based
//! on the middle values of reordered sequences.

// Standard library imports
use std::error::Error;

// Internal module imports
use calculations::process_sequences;
use errors::AppError;
use file_io::read_file_and_split;

mod calculations;
mod errors;
mod file_io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Day 5!");
    
    // Get input file path from command line arguments
    let path = std::env::args()
        .nth(1)
        .ok_or(AppError::ArgError("No input file provided"))?;

    // Read and parse input file
    let (ordering_rules, update_sequences) = read_file_and_split(&path)?;
    
    // Process sequences and calculate total
    let total = process_sequences(ordering_rules, update_sequences);
    println!("Total: {}", total);

    Ok(())
}
