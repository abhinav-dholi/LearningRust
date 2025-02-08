// // write a function the takes two strings as an input and returns the bigger amongst them

//  says missing lifetime specifier
// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

// fn main(){
//     let longest_str;
//     let str1 = String::from("abhinav");
//     {
//         let str2 = String::from("dholi_longest");
//         longest_str = longest(&str1, &str2);
//     }
//     print!("The longest string is: {}", longest_str);
// }

// Explanation:
// 1. Note that in the above piece of code in main -> str1 is outside of the brackets and str2 is inside the brackets.
// 2. The scope of str2 is limited to the brackets and hence str2 is not valid outside the brackets.
// 3. Keeping 2 in mind as the longest function returns a reference to the string and not borrowing the ownership of fhe string, the function returning the longest returns a pointer to the longest string `longest_str` which is str2 in this case.
// 4. As the `longest_str` holds the pointer to the longest string (not borrowed the ownership) i.e. str2, it is out of scope for the print statement and hence the error. (dangling reference)





fn main(){
    println!("This file explains the problem and why lifetimes are needed in Rust, for implementation look at lifetime_2/main.rs file");
}







// The Code below works fine as we are borrowing the ownership of the string and not returning the reference to the string.

// fn longest(a: String, b: String) -> String {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

// fn main() {
//     let longest_str;
//     let str1 = String::from("abhinav");
//     let str2 = String::from("dholi");
//     longest_str = longest(str1, str2);
//     println!("The longest string is: {}", longest_str); // longest_str is still valid here

//     let longest_str_new;
//     let str1_new = String::from("abhinav");
//     {
//         let str2_new = String::from("dholi");
//         longest_str_new = longest(str1_new, str2_new);
//     }
//     println!("The longest string is: {}", longest_str_new); // longest_str_new is still valid here
// }