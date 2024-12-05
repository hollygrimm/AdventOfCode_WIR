//! Module for processing and validating sequences according to ordering rules.

use std::collections::HashMap;

/// Processes a set of sequences according to ordering rules and calculates a total
/// based on the middle values of reordered sequences.
///
/// # Arguments
/// * `ordering_rules` - HashMap where key must appear before its associated values in sequences
/// * `update_sequences` - Sequences to validate and potentially reorder
///
/// # Returns
/// Sum of middle values from reordered invalid sequences
pub fn process_sequences(
    ordering_rules: HashMap<i32, Vec<i32>>,
    update_sequences: Vec<Vec<i32>>,
) -> i32 {
    let mut total = 0;

    for mut update in update_sequences {
        if !is_valid_sequence(&ordering_rules, &update) {
            reorder_sequence(&ordering_rules, &mut update);
            if let Some(middle_value) = find_middle_value(&update) {
                total += middle_value;
            }
        }
    }

    total
}

/// Checks if a sequence follows all ordering rules
///
/// # Arguments
/// * `ordering_rules` - Rules defining required ordering between numbers
/// * `update` - Sequence to validate
///
/// # Returns
/// `true` if sequence follows all rules, `false` otherwise
fn is_valid_sequence(ordering_rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    for (i, &key) in update.iter().enumerate() {
        if let Some(values) = ordering_rules.get(&key) {
            for &value in values {
                if let Some(pos) = update.iter().position(|&x| x == value) {
                    if pos <= i {
                        return false;
                    }
                }
            }
        }
    }
    true
}

/// Finds the middle value in a vector
///
/// # Arguments
/// * `update` - Vector to find middle value of
///
/// # Returns
/// The middle value if vector is non-empty, None otherwise
fn find_middle_value(update: &Vec<i32>) -> Option<i32> {
    let len = update.len();
    if len == 0 {
        None
    } else {
        Some(update[len / 2])
    }
}

/// Reorders a sequence to comply with ordering rules
///
/// Uses bubble sort approach to swap elements until all ordering rules are satisfied
///
/// # Arguments
/// * `ordering_rules` - Rules defining required ordering between numbers
/// * `update` - Sequence to reorder (modified in place)
fn reorder_sequence(ordering_rules: &HashMap<i32, Vec<i32>>, update: &mut Vec<i32>) {
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..update.len() {
            if let Some(values) = ordering_rules.get(&update[i]) {
                for &value in values {
                    if let Some(j) = update.iter().position(|&x| x == value) {
                        if j <= i {
                            update.swap(i, j);
                            changed = true;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_io::read_file_and_split;

    #[test]
    fn test_process_sequences() {
        let (ordering_rules, update_sequences) = read_file_and_split("data/inputtest").unwrap();
        let total = process_sequences(ordering_rules, update_sequences);
        assert_eq!(total, 123);
    }
}
