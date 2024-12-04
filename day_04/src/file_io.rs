use ndarray::Array2;

use crate::AppError;

/// Reads a file and converts its contents into a 2D array of characters.
/// Each line in the file becomes a row in the array.
///
/// # Arguments
///
/// * `filename` - Path to the input file
///
/// # Returns
///
/// * `Result<Array2<char>, AppError>` - A 2D array of characters from the file, or an error
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The file contains lines of different lengths
pub fn read_file(filename: &str) -> Result<Array2<char>, AppError> {
    let content = std::fs::read_to_string(filename)?;
    let lines: Vec<&str> = content.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let data: Vec<char> = lines.join("").chars().collect();
    Array2::from_shape_vec((rows, cols), data).map_err(|_| AppError::Array2CreationError)
}
