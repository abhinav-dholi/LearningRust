




fn main() {
    let first_name = String::from("Abhinav");
    let user = User {
        name: &first_name,
    };
    println!("{}", user.name);
}

// This struct will not compile as when first_name in main() gets out of scope then user should also go out of scope. But the struct User is holding a reference to the first_name which is out of scope. Hence the error.
// struct User {
//     name: &str,
// }


// This is how we can create a relationship between Struct user and name using lifetime

struct User<'a> {
    name: &'a str,
}

// This code tells us that the user struct will not outlive the reference to the name. The lifetime of the reference to the name is the same as the lifetime of the user struct. This is how we can create a relationship between the struct and the reference using lifetimes. This is how we can solve the error of dangling reference.