use std::io::{self};
use std::error::Error;
use std::fmt;
use once_cell::sync::Lazy;

// Regular expression to match multiplication expressions like mul(123,456)
static PRODUCT_RE: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap()
});

// Regular expression to match do, don't, and multiplication expressions
static DO_DONT_RE: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap()
});

/// Custom error type for the application
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ArgError(&'static str),
    ParseError(std::num::ParseIntError)
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

/// Calculates the total product of all multiplication expressions in the input string
///
/// # Arguments
///
/// * `input` - A string slice that holds the input data
///
/// # Returns
///
/// * `Result<i32, AppError>` - The total product or an error
fn calculate_products(input: &str) -> Result<i32, AppError> {
    let mut total = 0;
    
    for cap in PRODUCT_RE.captures_iter(input) {
        let num1: i32 = cap[1].parse()?;
        let num2: i32 = cap[2].parse()?;
        total += num1 * num2;
    }
    
    Ok(total)
}

/// Calculates the total product of all multiplication expressions in the input string
/// that are preceded by a "do()" and not by a "don't()"
///
/// # Arguments
///
/// * `input` - A string slice that holds the input data
///
/// # Returns
///
/// * `Result<i32, AppError>` - The total product or an error
fn calculate_products_do_dont(input: &str) -> Result<i32, AppError> {
    let mut total = 0;
    let mut should_add = true;
    
    for cap in DO_DONT_RE.captures_iter(input) {
        match &cap[1] {
            "do()" => should_add = true,
            "don't()" => should_add = false,
            _ => {
                if should_add {
                    let num1: i32 = cap[2].parse()?;
                    let num2: i32 = cap[3].parse()?;
                    total += num1 * num2;
                }
            }
        }
    }
    
    Ok(total)
}

/// Reads the content of a file into a string
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - The file content or an error
fn read_file_to_string(path: &str) -> Result<String, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    println!("Read {} bytes", content.len());
    Ok(content)
}

/// Main function to execute the program
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Success or an error
fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).ok_or(AppError::ArgError("No input file provided"))?;
    let input = read_file_to_string(&path)?;
    
    let total = calculate_products(&input)?;
    println!("Total sum of all products: {}", total);
    
    let total = calculate_products_do_dont(&input)?;
    println!("Total sum of all 'do' products: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the calculate_products function
    #[test]
    fn test_calculate_total() -> Result<(), Box<dyn Error>> {
        let input = read_file_to_string("data/inputtest")?;
        let total = calculate_products(&input)?;
        assert_eq!(total, 161, "Expected total to be 161, got {}", total);
        Ok(())
    }

    /// Tests the calculate_products_do_dont function
    #[test]
    fn test_calculate_products_do_dont() -> Result<(), Box<dyn Error>> {
        let input = read_file_to_string("data/inputtest")?;
        let total = calculate_products_do_dont(&input)?;
        assert_eq!(total, 48, "Expected total to be 48, got {}", total);
        Ok(())
    }
}
