// This is in continuation of lifetimes. Solving the error by lifetimes


// lifetime is used to specify the the lifespan `longest_str` in the above case. What we tell the compiler is the lifetime of longest string should end with the closing bracket of the scope of str2. This is done by specifying the lifetime of the reference in the function signature.

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

// the above function uses a `generic lifetime annotation`, first we define a `generic lifetime annotation` at `<'a>`, then we say that the lifetime of the first argument 'first' is `'a`, then we say that the lifetime of the second argument 'second' is also `'a` and then we say that the lifetime of the return value of the function is also `'a`. 

// This means that the lifetime of the return value is the INTERSECTION of the lifetime of the arguments. This is how we specify the lifetime of the reference in the function signature. This is how we solve the error of dangling reference.

fn main(){
    let longest_str;
    let str1 = String::from("abhinav");
    {
        let str2 = String::from("dholi_longest");
        longest_str = longest(&str1, &str2);
        println!("The longest string is: {}", longest_str);
    }
    // println!("The longest string is: {}", longest_str); // This will not work as now due to lifetime defined as INTERSECTION of str1 and str2, the lifetime of longest_str is till the closing bracket of str2
}


// Lifetime describes a relationship between the lifetimes of input arguments and output arguments. The lifetime of the return value is the intersection of the lifetimes of the input arguments. It says that the return type will be valid as long as the references passed in. OR more technically, the shorter lifetimes is what the return type will be.