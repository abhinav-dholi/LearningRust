fn main() {

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


// This is how the state machine would look like:

// enum FutureStateMachine {
//     State1,
//     State2,
//     State3,
// }

// Here is a step-by-step explanation of how the code runs
// First the first block of the code before the first await goes into the state machine State1 which gets executed synchronously
/// As the read_from_database takes time for State1 to be executed, the polling thread returns Pending until the read_from_database returns the result
/// Once the read_from_database returns the result, the next poll returns Ready, this information is given to the executor which causes wake to be called and the state machine to move to State2
/// The same process is repeated for State2 and State3
/// For this particular execution the total number of polls will be 3, one for each state
/// The number of polls can be more or less depending on the number of states and the time taken by each state to execute
/// 
/// Here is how the process would look like:
/// State1 will execute: poll 1
/// `println!("This is an async function");
    // let s1 = read_from_database().await;`
/// State2 will execute: poll 2
/// `println!("First Result: {}", s1);
    // let s2 = read_from_database().await;`
/// State3 will execute: poll 3
/// `println!("Second Result: {}", s2);`

// The wake call is used to notify the executor that the state machine is ready to be polled again
// The executor will then poll the state machine again and the process will continue until the state machine is complete
// The state machine is complete when the last state is executed
// The state machine can also be completed if the state machine returns Ready in the poll method.