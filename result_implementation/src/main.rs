// The result enum lets you return either an Ok value or the Err value. This is how error handling is done in Rust. The Ok value is the value that you want to return if the function is successful, and the Err value is the value that you want to return if the function fails.
use std::fs::read_to_string;

fn main() {
    let result = read_from_file_abhinav(String::from("a.txt"));

    match result {
        Ok(content) => println!("File content:\n{}", content),
        Err(error) => println!("Error: {}", error),
    }
}

// Returning a Result is a good practice
// because it makes it clear that the function can fail
fn read_from_file_abhinav(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);

    match result {
        Ok(content) => Ok(content),
        Err(error) => Err(format!("Failed to read file: {}", error)),
    }
}
