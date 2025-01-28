fn main() {
    let s1 = String::from("hello");
    do_something(&s1);
    println!("number from s1 is {}", s1);
}

fn do_something(s2: &String) {
    println!("number from s2 is {}", s2);
}

// there is a way to make the borrowing mutable by using &mut instead of $
// if there is one mutable reference, there can be no other references - ownership rule for memory safety
