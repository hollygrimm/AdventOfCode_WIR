use std::error::Error;

/// Reads the content of a file into a string
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - The file content or an error
pub fn read_file_to_string(path: &str) -> Result<String, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    println!("Read {} bytes", content.len());
    Ok(content)
}
