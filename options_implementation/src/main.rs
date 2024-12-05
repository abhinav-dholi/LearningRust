// option enum lets you return a value or None


fn main() {
    let index = find_first_a(String::from("Dholi"));

    match index {
        Some(i) => println!("The index of the first 'a' is {}", i),
        None => println!("There is no 'a' in the string")
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(i as i32);
        }
    }
    None
}
