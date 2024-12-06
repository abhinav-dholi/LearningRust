// you can add an external crate to your project by running cargo add crate_name 

// cargo add chrono 
use chrono::{Local, Utc};

fn main() {
    // get the current time in UTC format
    let now = Utc::now();
    println!("UTC now is: {}", now);

    // Format the data and time
    let formatted = now.format("%d-%m-%Y %H:%M:%S").to_string();
    println!("Formatted time is: {}", formatted);

    // get the local time in UTC format
    let local = Local::now();
    println!("Local now is: {}", local);
}
