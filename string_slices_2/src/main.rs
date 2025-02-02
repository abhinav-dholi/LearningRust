// write a fn that takes a string as an input and returns the first word from it.

fn main() {
    let input = String::from("Hello World!");
    println!("input is: {}", input);
    println!("ans is: {}", first_word(&input));
}

// approach 1 - Problem - we take up double the memory space and if the name gets cleared, 'ans' still has "Hello" as its value in it.
// this is a problem 
// fn first_word(str: String) -> String {
//     let mut ans = String::from("");
//     for i in str.chars() {
//         if i == ' ' {
//             break;
//         }
//         ans.push_str(&i.to_string());
//     }
//     ans
// }


// approach 2 - using string slices
// if we want to have a `view` into the string, we can use string slices and not copy it over - this is the correct way to do it.

fn first_word(str: &str) -> &str {
    let mut space_index = 0;
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        space_index += 1;
    }
    &str[..space_index]
}