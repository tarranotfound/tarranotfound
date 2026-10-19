struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let shape = Rectangle { width: 8, height: 5 };
    println!("Area: {}", shape.area());
    println!("Square: {}", shape.is_square());
}
