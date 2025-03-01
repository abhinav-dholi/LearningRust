use std::thread;
use std::time::Duration;


// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     thread::spawn(|| {
//     println!("Here's a vector: {:?}", v);
//     });
// }

// The error message is:
// error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
//   --> src/main.rs:10:29
//    |
// 10 |     thread::spawn(|| {
//    |                             ^^ may outlive borrowed value `v`
// 11 |         println!("Here's a vector: {:?}", v);
//    |                                            - `v` is borrowed here
//    |
// note: function requires argument type to outlive `'static`
//   --> src/main.rs:10:5
//    |
// 10 |     thread::spawn(|| {
//    |     ^^^^^^^^^^^^^^^
//    = note: the closure must outlive the current function, but the created `std::thread::Thread` only lives for the duration of the call
//
// error: aborting due to previous error
//
// For more information about this error, try `rustc --explain E0373`.
// error: could not compile `multithreading_3` due to previous error
//
// To learn more, run the command again with --verbose.
//
// The error message is telling us that the spawned thread may outlive the main thread, which would be a problem because the spawned thread references the vector v. Rust can't guarantee that v will still be valid when the spawned thread is running. To fix this error, we can use the move keyword before the closure, which forces the closure to take ownership of the values it's using rather than allowing Rust to infer that it should borrow the values. This is the code that fixes the error:

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
}
