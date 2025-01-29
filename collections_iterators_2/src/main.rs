// .into_iter() is used to convert a collection into an iterator that takes ownership of the collection and returns owned values. This is different from .iter() which borrows the collection and returns borrowed values. The code below demonstrates the use of .into_iter() method. This provides better performance than .iter() because it avoids the overhead of borrowing.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let iter = numbers.into_iter();
    for value in iter {
        println!("{}", value);
    }

    // println!("{:?}", numbers); -> this will not compile because numbers has been moved into the iterator

    // consuming an iterator -> methods that call next are called consuming adaptors, because calling them uses up the iterator. One example is the sum method which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator.

    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 15, "The total sum is incorrect!");
    println!("Assertion passed: total is {}", total);

    // let sum2: i32 = v1.iter().sum(); -> v1_iter cannot be used after this line because it has been moved into the sum method
}
