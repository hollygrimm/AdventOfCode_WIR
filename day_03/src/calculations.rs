use crate::errors::AppError;
use std::sync::LazyLock;

// Regular expression to match multiplication expressions like mul(123,456)
static PRODUCT_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

// Regular expression to match do, don't, and multiplication expressions
static DO_DONT_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap());

/// Calculates the total product of all multiplication expressions in the input string
///
/// # Arguments
///
/// * `input` - A string slice that holds the input data
///
/// # Returns
///
/// * `Result<i32, AppError>` - The total product or an error
pub fn calculate_products(input: &str) -> Result<i32, AppError> {
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
pub fn calculate_products_do_dont(input: &str) -> Result<i32, AppError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_io::read_file_to_string;
    use std::error::Error;

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
