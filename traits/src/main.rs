// A trait in rust defines the functionality a particular type has and can share with other types. Traits are similar to interfaces in other languages. A trait can be implemented for a type, and the type can use the functionality defined in the trait. Trait is similar to an abstract class in java. You can have a default implementation of a function in a trait -> you can override if you want to.

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait NewSummary {
    fn summarize_def(&self) -> String {
        String::from("Read more...")
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

impl NewSummary for User {
    // not their own implementation so it will use the default implementation
}

// Traits can also be used as parameters in functions. The function below takes a parameter of type Summary. This means that the function can take any type that implements the Summary trait. This is similar to generics, but the difference is that the function can only take types that implement the Summary trait.

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The `impl Trait` syntax is works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound. Remember GENERICS!!
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! from the trait bound syntax {}", item.summarize());
}

// Multiple trait bounds can be specified using the + syntax. For example, the function below takes a parameter that implements both the Summary and NewSummary traits.
fn notify_plus(item: &(impl Summary + NewSummary)) {
    println!("Breaking news! with 2 traits - Summary {}", item.summarize());
    println!("Breaking news! with 2 traits - NewSummary {}", item.summarize_def());
}

fn main() {
    let user = User {
        name: String::from("Abhinav"),
        age: 23,
    };
    println!("{}", user.summarize());
    println!("{}", user.summarize_def());
    notify(&user); // `impl Trait` syntax
    notify2(&user); // trait bound syntax

    notify_plus(&user); // this will not work because User does not implement NewSummary
}