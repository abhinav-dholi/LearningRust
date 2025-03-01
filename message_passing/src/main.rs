// message passing in rust is when you have two threads and you want to transfer data from one thread to another. The Rust standard library provides a channel type for message passing. The channel type is a multiple-producer, single-consumer queue. In this case, the producer is the thread that sends values and the consumer is the thread that receives those values. Here's an example of message passing using channels:
// mpsc -> multiple producer, single consumer

use std::sync::mpsc;
use std::thread;

fn spawn_producer(tx: mpsc::Sender<i32>) {
    thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
        }
    });
}
fn main() {
    let (tx, rx) = mpsc::channel();
    spawn_producer(tx);
    for received in rx {
        println!("Got: {received}");
    }
}


// 1. mpsc::channel creates a channel with two ends: Sender<T> and Receiver<T>.
// 2. The main thread creates a channel and passes the Sender<T> to the spawn_producer function.
// 3. The spawn_producer function spawns a new thread that sends values to the Sender<T>.
// 4. The main thread receives values from the Receiver<T> in a for loop.
// 5. The main thread prints the received values.
// 6. The program will exit when the spawned thread finishes sending values.
// The channel type is generic over T, which means you can send any type of value through a channel. The send method returns a Result<T, SendError<T>>. The Result<T, SendError<T>> type is an enum with two variants: Ok(T) and Err(SendError<T>). The Ok(T) variant contains the value that was sent, and the Err(SendError<T>) variant contains the value that failed to send. The SendError<T> type is a struct that contains the value that failed to send. The SendError<T> type implements the Debug trait, which means you can print the SendError<T> value using the {:?} format specifier. Here's an example of sending and receiving values through a channel:

// unwrap() means that the channel will panic if the send operation fails. This is useful for prototyping and testing, but in production code, you should handle the error gracefully.