use std::fmt::Display;

/// This function finds the longest of two string slices while also printing an announcement.
///
/// # Parameters:
/// - `'a` (Lifetime Annotation): Specifies that the returned reference will live 
///   at most as long as the shorter of `x` or `y`. This ensures safe borrowing.
/// - `x: &'a str`: A string slice with lifetime `'a`.
/// - `y: &'a str`: Another string slice with lifetime `'a`.
/// - `ann: T`: A generic parameter `T` which must implement the `Display` trait, 
///   allowing it to be printed.
///
/// # Returns:
/// - A reference to the longer of `x` or `y`, ensuring the returned reference does not 
///   outlive the shorter of the input lifetimes.
///
/// # Explanation:
/// - The function prints an announcement using the `Display` trait, which means 
///   `ann` can be any type that implements `fmt::Display` (e.g., a string, integer, etc.).
/// - It then compares the lengths of `x` and `y` and returns a reference to the longer one.
/// - Since both `x` and `y` share the same lifetime `'a`, Rust guarantees the returned reference 
///   is valid as long as the shortest-lived input reference.
/// 
/// /// # Important Notes:
/// - The function does not take ownership of `x`, `y`, or `ann`; it only borrows them.
/// - If the input lifetimes are mismatched (e.g., `x` lives longer than `y`), the compiler 
///   will ensure safe borrowing rules are followed to prevent returning an invalid reference.

fn longest_with_an_announcement<'a,T>(
    x: & 'a str,
    y: & 'a str,
    ann: T,
) -> & 'a str
where
    T: Display, // Where just tells the compiler that the trait being used in a Display Trait
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn main() {
    let str1 = String::from("abhinav");
    let str2 = String::from("dholi");
    let _result = longest_with_an_announcement(&str1, &str2, "This is the longest string!");
}

