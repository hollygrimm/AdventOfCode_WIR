use crate::errors::AppError;
use ndarray::Array2;

/// Searches for instances of a string pattern in an Array2 of characters.
/// The search is performed in all directions: horizontal, vertical, and diagonal.
/// The pattern can be found forwards or backwards.
///
/// # Arguments
///
/// * `input` - A 2D array of characters to search through
/// * `search` - The pattern to search for
///
/// # Returns
///
/// * `Result<i32, AppError>` - The number of pattern instances found, or an error
pub fn count_instances(input: &Array2<char>, search: &str) -> Result<i32, AppError> {
    let mut num_instances = 0;
    let (rows, cols) = input.dim();
    let search_len = search.len();
    let search_chars: Vec<char> = search.chars().collect();
    let search_reverse: Vec<char> = search_chars.iter().rev().cloned().collect();

    // Check rows
    for row in input.rows() {
        row.windows(search_len)
            .into_iter().filter(|window| {
                window.to_vec() == search_chars || window.to_vec() == search_reverse
            })
            .for_each(|_| num_instances += 1);
    }

    // Check columns
    for col in input.columns() {
        col.windows(search_len)
            .into_iter().filter(|window| {
                window.to_vec() == search_chars || window.to_vec() == search_reverse
            })
            .for_each(|_| num_instances += 1);
    }

    // Check diagonals
    for i in 0..rows {
        for j in 0..cols {
            // Down-right diagonal
            if i + search_len <= rows && j + search_len <= cols {
                let diag_chars: Vec<char> = (0..search_len).map(|k| input[[i + k, j + k]]).collect();
                if diag_chars == search_chars || diag_chars == search_reverse {
                    num_instances += 1;
                }
            }
            // Down-left diagonal
            if i + search_len <= rows && j >= search_len - 1 {
                let diag_chars: Vec<char> = (0..search_len).map(|k| input[[i + k, j - k]]).collect();
                if diag_chars == search_chars || diag_chars == search_reverse {
                    num_instances += 1;
                }
            }
        }
    }

    Ok(num_instances)
}

/// Searches for X-shaped patterns in an Array2 of characters.
/// An X-pattern consists of a three-character string where:
/// - The middle character is at the center
/// - The first and last characters form an X shape around the center
/// - The pattern can be read in either direction along both diagonals
///
/// # Arguments
///
/// * `input` - A 2D array of characters to search through
/// * `search` - A three-character string to search for
///
/// # Returns
///
/// * `Result<i32, AppError>` - The number of X-patterns found, or an error
///
/// # Example
/// For search string "MAS", valid X pattern would look like:
/// ```text
/// M   S
///   A
/// M   S
/// ```
pub fn count_x_instances(input: &Array2<char>, search: &str) -> Result<i32, AppError> {
    let mut num_instances = 0;
    let (rows, cols) = input.dim();

    // Need at least 3x3 area to form an X pattern
    if search.len() != 3 || rows < 3 || cols < 3 {
        return Ok(0);
    }

    let chars: Vec<char> = search.chars().collect();

    // Check each possible 3x3 grid center point
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Check if center is the middle character
            if input[[i, j]] == chars[1] {
                // Check top-left to bottom-right corners
                if (input[[i - 1, j - 1]] == chars[0] && input[[i + 1, j + 1]] == chars[2])
                    || (input[[i - 1, j - 1]] == chars[2] && input[[i + 1, j + 1]] == chars[0])
                {
                    // Check top-right to bottom-left corners
                    if (input[[i - 1, j + 1]] == chars[0] && input[[i + 1, j - 1]] == chars[2])
                        || (input[[i - 1, j + 1]] == chars[2] && input[[i + 1, j - 1]] == chars[0])
                    {
                        num_instances += 1;
                    }
                }
            }
        }
    }

    Ok(num_instances)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_io::read_file;
    use std::error::Error;

    /// Tests the count_instances function
    #[test]
    fn test_num_xmas_instances() -> Result<(), Box<dyn Error>> {
        let input = read_file("data/inputtest")?;
        let num_xmas_instances = count_instances(&input, "XMAS")?;
        assert_eq!(
            num_xmas_instances, 18,
            "Expected total to be 18, got {}",
            num_xmas_instances
        );
        Ok(())
    }

    /// Tests the count_x_instances function
    #[test]
    fn test_num_x_mas_instances() -> Result<(), Box<dyn Error>> {
        let input = read_file("data/inputtest")?;
        let num_x_mas_instances = count_x_instances(&input, "MAS")?;
        assert_eq!(
            num_x_mas_instances, 9,
            "Expected total to be 9, got {}",
            num_x_mas_instances
        );
        Ok(())
    }
}
