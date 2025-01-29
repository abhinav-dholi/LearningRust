// iterator adaptors are methods defined on the Iterator trait that allow us to change iterators into different kinds of iterators. They are extensively used in the standard library to perform complex actions in a readable and reusable way. The code below demonstrates the use of iterator adaptors.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();
    // iter2 maps each element of numbers_iter to a new element by adding 1 to it
    let iter2 = numbers_iter.map(|x| x + 1);
    for value in iter2 {
        println!("{}", value);
    }

    // the original vector is still intact
    println!("{:?}", numbers);

    // filter vector

    let numbers2: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numbers2_iter = numbers2.iter();
    let iter3 = numbers2_iter.filter(|x| *x % 2 == 0);
    for value in iter3 {
        println!("{}", value);
    }
}
