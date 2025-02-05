// generics is a concept that exists in many languages, including Rust and typescript. Generics are used to represent a type that represents many types. For example, a function that returns the largest number between two numbers can be written in a generic way. This way, the function can be used to compare numbers, characters, or any other type that implements the PartialOrd trait.

fn main() {
    let bigger = largest(1,2);
    let bigger_char = largest('a','b');
    println!("The biggest number is {}", bigger);
    println!("The biggest character is {}", bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
