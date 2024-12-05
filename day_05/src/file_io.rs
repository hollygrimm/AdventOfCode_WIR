use crate::errors::AppError;
use std::collections::HashMap;

/// Reads the content of a file and splits it on double new lines.
/// Returns ordering rules and updates
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file
///
/// # Returns
///
/// * `Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>), AppError>` - A tuple containing a hashmap of ordering rules and a vector of update sequences or an error
pub fn read_file_and_split(
    path: &str,
) -> Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>), AppError> {
    let content = std::fs::read_to_string(path)?;
    println!("Read {} bytes", content.len());
    // Split the input file into sections based on double newlines
    let sections: Vec<&str> = content.split("\n\n").collect();

    // Parse the first section into ordering rules
    // Format: key|value where value must come after key in sequences
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    if let Some(first_section) = sections.get(0) {
        for line in first_section.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let key = parts[0].parse().map_err(AppError::ParseError)?;
                let value = parts[1].parse().map_err(AppError::ParseError)?;
                ordering_rules
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(value);
            }
        }
    }

    // Parse the second section into sequences that need to be validated/reordered
    // Format: comma-separated integers representing update sequences
    let mut update_sequences: Vec<Vec<i32>> = Vec::new();
    if let Some(second_section) = sections.get(1) {
        for line in second_section.lines() {
            if !line.is_empty() {
                let update_sequence: Vec<i32> = line
                    .split(',')
                    .map(|s| s.parse().map_err(AppError::ParseError))
                    .collect::<Result<_, _>>()?;
                update_sequences.push(update_sequence);
            }
        }
    }

    Ok((ordering_rules, update_sequences))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_and_split() -> Result<(), AppError> {
        let (ordering_rules, update_sequences) = read_file_and_split("data/inputtest")?;

        // Test ordering rules
        assert_eq!(ordering_rules.get(&47), Some(&vec![53, 13, 61, 29]));
        assert_eq!(ordering_rules.get(&97), Some(&vec![13, 61, 47, 29, 53, 75]));
        assert_eq!(ordering_rules.get(&75), Some(&vec![29, 53, 47, 61, 13]));

        // Test update sequences
        let expected_sequences = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(update_sequences, expected_sequences);

        Ok(())
    }
}
