//! A program that processes pairs of numbers from stdin, sorts them, and calculates
//! the sum of absolute differences between corresponding elements.
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
//! 1 5
//! 2 3
//! 10 15
//! <Ctrl+D>
//! ```

use std::io::{self, BufRead};
use std::error::Error;

/// Maximum allowed value for any input number
const MAX_VALUE: i32 = 100_000;
/// Maximum allowed size for the input lists
const MAX_LIST_SIZE: usize = 1000;

/// Main function that reads number pairs from stdin, validates them,
/// sorts the resulting lists, and calculates the sum of absolute differences.
///
/// # Error Handling
/// Returns an error if:
/// - Input cannot be read or parsed
/// - A line doesn't contain exactly 2 numbers
/// - Any number is >= MAX_VALUE
/// - Input exceeds MAX_LIST_SIZE pairs
///
/// # Example Input Format
/// ```text
/// 1 5
/// 2 3
/// 10 15
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    // Pre-allocate vectors with maximum capacity for better performance
    let mut list1 = Vec::with_capacity(MAX_LIST_SIZE);
    let mut list2 = Vec::with_capacity(MAX_LIST_SIZE);

    // Read and validate input line by line
    for line in io::stdin().lock().lines() {
        let line = line?;
        // Parse space-separated numbers into a vector
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        
        // Validate that we have exactly two numbers per line
        if numbers.len() != 2 {
            return Err("Each line must contain exactly 2 numbers".into());
        }
        
        // Validate that numbers don't exceed MAX_VALUE
        if numbers[0] >= MAX_VALUE || numbers[1] >= MAX_VALUE {
            return Err(format!("Input contains numbers >= {}", MAX_VALUE).into());
        }
        
        // Add numbers to their respective lists
        list1.push(numbers[0]);
        list2.push(numbers[1]);
        
        // Check if we've exceeded the maximum list size
        if list1.len() > MAX_LIST_SIZE {
            return Err(format!("Lists must not exceed {} elements", MAX_LIST_SIZE).into());
        }
    }

    // Sort both lists using sort_unstable (faster than stable sort when ordering of equal elements doesn't matter)
    list1.sort_unstable();
    list2.sort_unstable();

    // Calculate the sum of absolute differences between corresponding elements
    let total: i32 = list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| (*a - *b).abs())
        .sum();

    // Output the result
    println!("Total: {total}");
    Ok(())
}
