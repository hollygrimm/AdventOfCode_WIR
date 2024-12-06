mod calculations;
mod file_io;
mod errors;

use calculations::count_guard_path;
use file_io::read_file;
use errors::AppError;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Day 6!");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(Box::new(AppError::ArgError("Please provide a file path as argument")));
    }

    let file_path = &args[1];
    let contents = read_file(file_path)?;
    let result = count_guard_path(contents)?;
    
    println!("Result: {}", result);
    
    Ok(())
}
