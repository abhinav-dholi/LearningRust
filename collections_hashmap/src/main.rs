use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert("Alice", 30);
    users.insert("Bob", 25);
    users.insert("Charlie", 20);

    let first_user_age = users.get("Alice");

    match first_user_age {
        Some(age) => println!("Alice is {} years old", age),
        None => println!("Alice is not found"),
    }
}
