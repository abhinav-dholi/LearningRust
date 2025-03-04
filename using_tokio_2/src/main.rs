// In this implementation we will run the tokio command concurrently using tasks. Tasks in tokio are lightweight units of work that are executed concurrently. This means that we can run multiple tasks concurrently without having to worry about multiple tasks blocking each other.
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] This is an async function");
    let s1 = read_from_database().await;
    println!("[{i}] First Result: {}", s1);
    let s2 = read_from_database().await;
    println!("[{i}] Second Result: {}", s2);
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}

// we can also force tokio to only use one thread at a time by using the following command:
// #[tokio::main(flavor = "current_thread")]

/*
    This Rust program demonstrates asynchronous concurrency using the Tokio runtime.

    Key Concepts:
    1. **Tokio Tasks (`tokio::spawn`)**:
       - Tasks in Tokio are lightweight units of work that execute concurrently.
       - The `tokio::spawn` function is used to spawn asynchronous tasks that run independently.

    2. **Async/Await**:
       - Functions are marked as `async` to enable asynchronous execution.
       - The `await` keyword is used to yield control until the asynchronous function completes.

    3. **Concurrency with Tasks**:
       - A loop creates multiple asynchronous tasks (`tokio::spawn`), each calling `my_function(i)`.
       - These tasks run concurrently without blocking the main thread.

    4. **Awaiting Task Completion**:
       - A vector `handles` stores task handles.
       - The `handle.await.unwrap();` ensures all tasks complete before `main` exits.

    5. **Simulating Asynchronous Database Reads**:
       - `read_from_database()` simulates a database call using `tokio::time::sleep()`.
       - This introduces a non-blocking delay, mimicking real-world I/O operations.

    6. **Using the Tokio Runtime**:
       - `#[tokio::main]` initializes and runs the async runtime.
       - The alternative `#[tokio::main(flavor = "current_thread")]` forces a single-threaded runtime.

    Summary:
    - The program spawns two concurrent tasks that call `my_function(i)`, each making two asynchronous database reads.
    - The execution remains efficient since `tokio::spawn` allows non-blocking concurrency.
    - The `sleep(Duration::from_millis(50)).await` simulates a realistic async delay.
*/