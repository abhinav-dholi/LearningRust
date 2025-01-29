// iterator pattern allows us to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When we use iterators, we don’t have to reimplement that logic ourselves.

// In rust iterators are lazy, so we no affect until we call methods that consume the iterator to use it up. For example, the code below will not print anything because we never call any methods that consume the iterator.

// treat it like a data structure

// the standard syntax of going through an array of items like `for value in numbers` is actually using an iterator behind the scenes.

// .iter method borrows the collection and returns an iterator that borrows each element of the collection.
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let _iter = numbers.iter();

    for value in _iter {
        print!("{} ", value);        
    }
    println!();

    // iterate using .next() method -> this is the expected way of using an iterator
    let numbers2 = vec![1, 2, 3, 4, 5];
    let mut iter = numbers2.iter();
    while let Some(value) = iter.next() {
        println!("{} ", value);
    }

    // using a mutable iterator
    let mut numbers3 = vec![1, 2, 3, 4, 5];
    let iter = numbers3.iter_mut();

    for value in iter {
        *value += 10;
    }
    println!("{:?}", numbers3);

    
}
