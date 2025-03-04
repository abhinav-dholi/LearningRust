fn main() {

}


// await keyword is used to wait for the result of an asynchronous function

/*
    ┌─────────────────────────────────────────────────────────────────────────────────────────┐
    │                              EXPLANATION OF `Future` TRAIT                               │
    ├─────────────────────────────────────────────────────────────────────────────────────────┤
    │  In Rust, `async fn` is a convenient way to define asynchronous functions.              │
    │  Under the hood, `async fn` is transformed into a function that returns a `Future`.    │
    │                                                                                         │
    │  # How `async fn` is Transformed:                                                      │
    │  - The function `async fn my_function()` is equivalent to `fn my_function() -> impl Future<Output=()>`. │
    │  - Instead of executing immediately, it returns a `Future` that must be **polled**.    │
    │  - The function's body is wrapped inside an **anonymous future** that runs later.      │
    │                                                                                         │
    │  # The `Future` Trait                                                                  │
    │  - The `Future` trait defines an **asynchronous computation** that can be polled.      │
    │  - It has an associated type `Output`, which represents the eventual result.           │
    │  - The method `poll(&mut self, wake: fn()) -> Poll<Self::Output>` drives execution.    │
    │  - `poll` does not block; it returns `Poll::Pending` if the computation isn’t ready.   │
    │  - Once the computation completes, `poll` returns `Poll::Ready(result)`.               │
    │                                                                                         │
    │  # The `Poll` Enum                                                                     │
    │  - `Poll<T>` is used to represent the state of a `Future`.                             │
    │    - `Poll::Ready(T)`: The computation has finished, and `T` is the result.            │
    │    - `Poll::Pending`: The computation is still running and must be re-polled later.    │
    │                                                                                         │
    │  # Key Concepts in Async Execution:                                                   │
    │  - A `Future` is lazy; it does nothing until an executor (e.g., Tokio, async-std)      │
    │    drives it forward by calling `poll()`.                                              │
    │  - The executor ensures that when a `Future` is **pending**, it gets re-polled when   │
    │    progress can be made (e.g., when I/O is ready).                                     │
    └─────────────────────────────────────────────────────────────────────────────────────────┘
*/

// Example of `Future` trait and `Poll` enum:

trait Future {
    type Output;

    // The `poll` method is used to drive the asynchronous execution of the future.
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

// The `Poll` enum represents the state of the future.
enum Poll<T> {
    Ready(T),  // The computation is complete.
    Pending,   // The computation is still in progress.
}

// Example of `async fn` transformation:

async fn my_function() {
    println!("This is an async function");
}

// The above is equivalent to:
fn my_function() -> impl Future<Output = ()> {
    async {
        println!("This is an async function");
    }
}