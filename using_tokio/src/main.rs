// For running the asynchronous code, we need an executor like Tokio or async-std.
// Tokio provides a separate executor for running asynchronous code. It is a runtime library that provides asynchronous I/O, timers, and a task scheduler.

// first we need to add tokio as a dependency to the Cargo.toml file:
// [dependencies]
// tokio = { version = "1", features = ["full"] }
//
// The features = ["full"] line is used to enable all the features of the tokio crate.
// The tokio crate provides a runtime that can be used to run asynchronous code.


#[tokio::main] // this is an attribute macro that allows our main function to be asynchronously executed under the tokio runtime environment

async fn main() {
    // my_function().await; // we call our async function and await its completion before proceeding with the rest of the code.
    let f = my_function();
    println!("This is the main function"); // This will be printed before the async function completes which shows that the function is running asynchronously
    f.await;
}

async fn my_function() {
    println!("This is an async function");
    let s1 = read_from_database().await;
    println!("First Result: {}", s1);
    let s2 = read_from_database().await;
    println!("Second Result: {}", s2);
}

async fn read_from_database() -> String {
    "DB Result".to_owned()
}

// One benefit of Future being lazy is that they are a zero-cost abstraction. This means that it will not use any runtime resources until it is polled.
// This is in contrast to threads which consume resources even when they are not doing anything.
// Another benefit of Futures being lazy is that they can be cancelled, which can be done by simply dropping the Future. Or simply not polling it.
// This is in contrast to threads which cannot be cancelled once they are started.
// This makes Futures a very efficient way to handle asynchronous code.

// Well in this execution we are not taking any benefit of tokio as we are running everything serially.