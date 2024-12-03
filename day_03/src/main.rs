use std::error::Error;

// Internal imports
mod calculations;
mod errors;
mod file_io;

use calculations::{calculate_products, calculate_products_do_dont};
use errors::AppError;
use file_io::read_file_to_string;

/// Main function to execute the program
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Success or an error
fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or(AppError::ArgError("No input file provided"))?;
    let input = read_file_to_string(&path)?;

    let total = calculate_products(&input)?;
    println!("Total sum of all products: {}", total);

    let total = calculate_products_do_dont(&input)?;
    println!("Total sum of all 'do' products: {}", total);
    Ok(())
}
