use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); // wait for the spawned thread to finish, unwrap() panics if the thread has panicked which means the thread has encountered an unrecoverable error for example a panic! macro call. 
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}


// 1. thread::spawn creates a new thread and returns a handle to it.
// 2. The new thread will execute the closure we passed to thread::spawn.
// 3. The main thread will continue to execute the code after thread::spawn.
// 4. The new thread will print "hi number i from the first thread!" i times.
// 5. The main thread will print "hi number i from the main thread!" i times.
// 6. The main thread will wait for the new thread to finish.
// 7. The main thread will print "hi number i from the main thread!" i times.
// 8. The main thread will wait for the new thread to finish.
// 9. The main thread will finish executing.
// 10. The new thread will finish executing.
// 11. The program will exit.