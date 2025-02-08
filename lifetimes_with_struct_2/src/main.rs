// Here is one more example for lifetime with struct

struct User<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str, 
}

// the lifetime of the reference to the first_name is 'a and the lifetime of the reference to the last_name is 'b. This means that the lifetime of the reference to the first_name is not dependent on the lifetime of the reference to the last_name. This is how we can specify multiple lifetimes in a struct. 

fn main() {
    let first_name = String::from("Abhinav");
    {
        let last_name = String::from("Dholi");
        let user = User {
            first_name: &first_name,
            last_name: &last_name,
        };
    }
    // println!("The first name is {}", user.first_name); // this will not work because the lifetime of first_name is 'a and the lifetime of last_name is 'b, allowing them to be independently borrowed. However, the struct as a whole cannot outlive the shortest of these lifetimes (in this case, 'b). If a shorter-lived reference is used, the struct itself will be invalidated once that reference goes out of scope.
}
