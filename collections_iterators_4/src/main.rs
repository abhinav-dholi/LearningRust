// write a logic to first filter all odd values then double them and create a new vector

use std::collections::HashMap;
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", filter_and_map(numbers));

    // iterators can also be used in HashMaps
    let mut people = HashMap::new();
    people.insert("Alice", 28);
    people.insert("Bob", 22);
    people.insert("Carol", 31);

    // collect is used to convert an iterator into a vector

    for (key, value) in people.iter() {
        println!("{}: {}", key, value);
    }
    let names: Vec<&str> = people.keys().map(|name| *name).collect();
    println!("{:?}", names);

    // iterating over mutable key value pairs

    for (key, age) in people.iter_mut() {
        *age += 1;
        println!("{}: {}", key, age);
    }

}

// collect is used to convert an iterator into a vector
fn filter_and_map(v: Vec<i32>) -> Vec<i32> {
    v.iter().filter(|x| *x % 2 == 1).map(|x| *x * 2).collect()
}