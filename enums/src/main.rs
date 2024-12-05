enum Shapes {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let rect = Shapes::Rectangle(1.0, 2.0);
    println!("area is {}", calculate_area(rect));
    
    let circle = Shapes::Circle(3.0);
    println!("area is {}", calculate_area(circle));
}

fn calculate_area(shape: Shapes) -> f64 {
   match shape {
        Shapes::Rectangle(width, height) =>  width * height,
        Shapes::Circle(radius) =>  3.14 * radius * radius,
    }
}