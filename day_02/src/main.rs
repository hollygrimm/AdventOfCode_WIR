//! A program that processes a set of numbers from stdin, each line is a report.
//! Each report contains a list of space-separated numbers called "levels".
//! 
//! A report is considered "safe" if:
//! 1. The levels are strictly monotonic (either all increasing or all decreasing)
//! 2. The difference between any two adjacent levels is between 1 and 3 (inclusive)
//! 3. OR if removing exactly one level makes the report satisfy conditions 1 and 2
//!
//! # Running the Program
//! From the project root directory, you can run the program in several ways:
//!
//! 1. Using input.txt:
//! ```bash
//! cargo run < data/input.txt
//! ```
//!
//! 2. Using manual input (press Ctrl+D or Ctrl+Z when finished):
//! ```bash
//! cargo run
//! 7 6 4 2 1
//! 1 2 7 8 9
//! 9 7 6 2 1
//! 1 3 2 4 5
//! 8 6 4 4 1
//! 1 3 6 7 9
//! <Ctrl+D>
//! ```

use std::io::{self};
use std::error::Error;

/// Custom error type for the application
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
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

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

/// Validates if a sequence of levels forms a safe report
/// 
/// # Arguments
/// * `levels` - A slice of integers representing the levels in a report
///
/// # Returns
/// * `true` if:
///   - All numbers are strictly increasing or strictly decreasing
///   - Each adjacent pair differs by 1, 2, or 3
/// * `false` otherwise
fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut prev = levels[0];
    let first_diff = levels[1] - prev;
    let is_increasing = first_diff > 0;

    for &current in &levels[1..] {
        let diff = current - prev;
        let diff_abs = diff.abs();
        
        // if two adjacent levels are the same or
        // differ more than 3, report is unsafe
        if diff_abs < 1 || diff_abs > 3 {
            return false;
        }
        
        // If direction changes, report is unsafe
        if (diff > 0) != is_increasing {
            return false;
        }
        
        prev = current;
    }

    true
}

/// Processes reports from standard input and counts how many are "safe"
///
/// A report is considered safe if:
/// - It's safe according to `is_safe_report`, or
/// - Removing exactly one level makes it safe
///
/// # Error Handling
/// Returns an error if:
/// - There's an IO error while reading input
/// - Any number in the input cannot be parsed as an integer
///
/// # Example Input Format
/// ```text
/// 7 6 4 2 1    # Safe: strictly decreasing, differences ≤ 3
/// 1 2 7 8 9    # Unsafe: strictly increasing, differences > 3
/// 9 7 6 2 1    # Unsafe: strictly decreasing, differences > 3
/// 1 3 2 4 5    # Safe with dampener: by removing 3, strictly increasing, differences ≤ 3 
/// 8 6 4 4 1    # Safe with dampener: by removing 4, strictly decreasing, differences > 0 
/// 1 3 6 7 9    # Safe: strictly increasing, differences ≤ 3
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    let mut safe_count = 0;
    let stdin = io::stdin();
    let mut buffer = String::new();

    // Read and validate reports line by line, each report has one or more levels
    while stdin.read_line(&mut buffer)? > 0 {
        let levels: Vec<i32> = buffer
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        
        #[cfg(debug_assertions)]
        println!("Read levels: {:?}", levels);

        if is_safe_report(&levels) {
            #[cfg(debug_assertions)]
            println!("safe without dampener");
            safe_count += 1;
        // if not safe, see if removing one level can make it safe
        } else if levels.len() > 2 {
            // Preallocate vector with capacity
            let mut modified_levels = Vec::with_capacity(levels.len() - 1);
            for i in 0..levels.len() {
                modified_levels.clear();
                modified_levels.extend(levels[..i].iter().chain(levels[i + 1..].iter()));
                
                if is_safe_report(&modified_levels) {
                    #[cfg(debug_assertions)]
                    println!("safe with dampener");
                    safe_count += 1;
                    break;
                }
            }
        }
        
        buffer.clear();
    }

    println!("Number of safe reports: {}", safe_count);

    Ok(())
}
