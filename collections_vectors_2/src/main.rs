// A vector's type can be inferred from the values it contains. OR using generic type you can specify the type of values it will contain.

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    let fruits: Vec<String> = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    println!("{:?}", fruits);

    let empty_vec: Vec<i32> = Vec::new();
    println!("{:?}", empty_vec);

    let mut mutable_vec = Vec::new();
    mutable_vec.push(1);
    mutable_vec.push(2);
    mutable_vec.push(3);
    println!("{:?}", mutable_vec);

    let mut mutable_vec2 = vec![1, 2, 3];
    mutable_vec2.push(4);
    mutable_vec2.push(5);
    println!("{:?}", mutable_vec2);
}
 