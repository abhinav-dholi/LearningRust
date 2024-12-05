// The result enum lets you return either an Ok value or the Err value. This is how error handling is done in Rust. The Ok value is the value that you want to return if the function is successful, and the Err value is the value that you want to return if the function fails.
use std::fs::read_to_string;

fn main() {
    let result = read_to_string("a.txt");

    match result {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }
}
