#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let desk = Rectangle { width: 120, height: 70 };
    let notebook = Rectangle { width: 20, height: 28 };
    println!("Desk: {desk:?}, area: {}", desk.area());
    println!("Notebook fits: {}", desk.can_fit(&notebook));
}
