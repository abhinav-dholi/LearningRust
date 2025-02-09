struct Rect {
    width: u32,
    height: u32,
}

// this in a js class 
/*
    class Rect {
    constructor(width, height) {
        this.width = width;
        this.height = height;
    }
    area() {
        return this.width * this.height;
    }
    perimeter() {
        return 2 * (this.width + this.height);
    }
    static debug() {
        return 1;
    }

    const rect1 = new Rect(10, 20)
    console.log(rect1.area())
    console.log(rect1.perimeter())
    console.log(Rect.debug()) // this will return 1
 */

 // this is how we attach a function to a struct 

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    // this is a static method in rust, it does not require an instance of the struct
    fn debug()-> u32 {
        return 1
    }
}


fn main() {
    let rect = Rect { width: 30, height: 50 };
    println!("Area of the rectangle: {}", rect.area());
    println!("Perimeter of the rectangle: {}", rect.perimeter());
    println!("debug of the rectangle: {}", Rect::debug());
}