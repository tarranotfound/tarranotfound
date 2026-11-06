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
    let card = Rectangle { width: 12, height: 8 };
    println!("Area: {}", card.area());
    println!("Square: {}", card.is_square());
}