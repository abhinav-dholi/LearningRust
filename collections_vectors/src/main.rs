// vectors allow you to store more than one kind of value in one data structure that puts alll the values next to each other in memory

fn main() {
    let mut numbers = Vec::new();

    numbers.push(3);
    numbers.push(4);
    numbers.push(5);

    // using the macro to create a vector
    let mut numbers2 = vec![1, 2, 3, 4, 5];

    // implementing a debug trait to print the vector
    println!("{:?}", even_filter(&numbers));



}

fn even_filter(vector: &Vec<i32>) -> Vec<i32> {
    let mut even_numbers = Vec::new();

    for number in vector {
        if number % 2 == 0 {
            // dereference the pointer with * to get the value inside the vector element, then push it into the new vector. '*' is for dereferencing the pointer
            even_numbers.push(*number);
        }
    }

    even_numbers
}