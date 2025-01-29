// write a function that takes a vector of tuples (each tuple containing a key and a value) and returns 
// a hashmap where the keys are the unique keys from the input tuples and the values are vectors of all corresponding values associates with that key.

use std::collections::HashMap;
fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for (key, value) in vec {
        map.insert(key, value);
    }
    map
}

fn main() {
    let tuples = vec![("apple".to_string(), 5), ("banana".to_string(), 3), ("abhinav".to_string(), 7), ("cherry".to_string(), 2)];
    let map = group_values_by_keys(tuples);

    println!("{:?}", map);
}
