// struct is very similar to class in js -> they can have methods

struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    let user = User {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };

    println!("User: {} {} is {} years old", user.first_name, user.last_name, user.age);
}
