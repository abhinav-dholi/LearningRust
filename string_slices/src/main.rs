// The String Type which is provided by the Rust Standard Library is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans reger to "strings" in Rust, they usually mean the String and the string slice &str types, not just one of those types. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection - they are a more generic concept. For example, you can have a slice of a String, a vector, or an array.

fn main() {
    let mut name = String::from("Abhinav");
    name.push_str(" Dholi");
    println!("name is {}", name);
    name.replace_range(7..name.len(), " Romy Dholi");
    println!("name is {}", name);

}

