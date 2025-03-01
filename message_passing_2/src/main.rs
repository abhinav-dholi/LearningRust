// Write a code that finds the sum from 1 - 10^8. Use threads to make sure that you use all cores of your machine.

// Importing necessary modules
use std::sync::mpsc; // Multi-producer, single-consumer channel for message passing
use std::thread;     // For spawning threads

fn main() {
    // Creating a channel for inter-thread communication
    let (tx, rx) = mpsc::channel(); // `tx` is the sender, `rx` is the receiver

    // Spawning 10 threads to compute partial sums
    for i in 0..10 {
        let producer = tx.clone(); // Clone the sender for each thread

        // Spawning a new thread
        thread::spawn(move || {
            let mut ans: u64 = 0; // Variable to store the partial sum

            // Correcting the range calculation
            let start = i * 10_000_000 + 1;
            let end = (i + 1) * 10_000_000;

            for j in start..=end { // Ensure inclusive range
                ans += j;
            }

            // Sending the computed partial sum to the main thread
            producer.send(ans).unwrap(); // `unwrap()` ensures the send operation succeeds
        });
    }
    drop(tx); // Dropping the sender to close the channel

    let mut ans: u64 = 0; // Variable to store the final sum

    // Collecting results from all threads
    for val in rx {
        ans += val; // Adding each received partial sum to the total sum
        println!("Received partial sum");
    }

    // Printing the final sum
    println!("Ans is: {}", ans);
}

// The program calculates the sum of numbers from 1 to 10⁸ by leveraging multithreading to utilize all CPU cores efficiently. It divides the range into 10 equal parts, with each thread computing the sum of a 10 million segment. A multi-producer, single-consumer (mpsc) channel is used for inter-thread communication, where each thread sends its computed partial sum to the main thread. The main thread receives these values, aggregates them into a final sum, and prints the result. This approach speeds up computation by distributing the workload across multiple  threads.