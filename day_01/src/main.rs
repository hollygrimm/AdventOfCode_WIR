//! A program that processes pairs of numbers from stdin, sorts them, and calculates
//! 1) the sum of absolute differences between corresponding elements
//! 2) the total similiarity score by taking each element in the first list and multiplying
//! the value times the number of times the the number appears in the second list.
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
use std::collections::HashMap;

/// Maximum allowed value for any input number
const MAX_VALUE: i32 = 100_000;
/// Maximum allowed size for the input lists
const MAX_LIST_SIZE: usize = 1000;

/// Custom error type for the application
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    InvalidPairCount,
    ValueTooLarge(i32),
    ListTooLong(usize),
}

impl std::error::Error for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
            Self::InvalidPairCount => write!(f, "Each line must contain exactly 2 numbers"),
            Self::ValueTooLarge(max) => write!(f, "Input contains numbers >= {}", max),
            Self::ListTooLong(max) => write!(f, "Lists must not exceed {} elements", max),
        }
    }
}

/// Main function that reads number pairs from stdin, validates them,
/// sorts both lists, and calculates the sum of absolute differences and
/// the total similiarity score.
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
    let mut list1 = Vec::with_capacity(MAX_LIST_SIZE);
    let mut list2 = Vec::with_capacity(MAX_LIST_SIZE);

    // Read and validate input line by line
    for line in io::stdin().lock().lines() {
        let line = line.map_err(AppError::IoError)?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().map_err(AppError::ParseError))
            .collect::<Result<_, _>>()?;
        
        if numbers.len() != 2 {
            return Err(Box::new(AppError::InvalidPairCount));
        }
        
        if numbers[0] >= MAX_VALUE || numbers[1] >= MAX_VALUE {
            return Err(Box::new(AppError::ValueTooLarge(MAX_VALUE)));
        }

        if list1.len() >= MAX_LIST_SIZE {
            return Err(Box::new(AppError::ListTooLong(MAX_LIST_SIZE)));
        }
        
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    // Sort both lists using sort_unstable (faster than stable sort when ordering of equal elements doesn't matter)
    list1.sort_unstable();
    list2.sort_unstable();

    // Combine frequency map calculation and sum calculation
    let mut frequency_map = HashMap::new();
    let mut sum_of_products = 0;

    // Build frequency map
    for &number in &list2 {
        *frequency_map.entry(number).or_insert(0) += 1;
    }

    // Optional debug output
    #[cfg(debug_assertions)]
    for (number, count) in &frequency_map {
        println!("Number {} appears {} times in list2", number, count);
    }

    // Calculate the sum of absolute differences between corresponding elements
    let total: i32 = list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| (*a - *b).abs())
        .sum();

    // Output the result
    println!("Total: {total}");

    // Calculate sum using the frequency map
    for &num in &list1 {
        if let Some(&count) = frequency_map.get(&num) {
            sum_of_products += num * count;
        }
    }

    println!("Sum of products: {}", sum_of_products);
    Ok(())
}
