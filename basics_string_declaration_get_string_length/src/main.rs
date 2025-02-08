fn main() {
    let name = String::from("abhinav");
    let length = get_string_length(name);
    println!("Length of the string: {}", length);
}

fn get_string_length(str: String) -> usize {
    // it does an implicit return if I do not put in a semicolon
    str.len()
}

