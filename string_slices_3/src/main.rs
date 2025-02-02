// here are the most commonly used strings

fn main() {
    let _name = String::from("Abhinav is the best!"); // String type
    let _string_slice = &_name; // Has a `view` into the original string/is a reference
    let _string_literal = "Abhinav is the best!"; // string literal is also a &str but ut points directly to a reference in the binary.

    // slices can also be applied to vectors or other collections
    let vector = vec![1, 2, 3, 4, 5];
    let vector_slice = &vector[..]; // slice of the vector
    println!("{}", vector_slice[0]); // 1
}
