fn main() {
    let point: (i32, i32) = (8, 13);
    let (x, y) = point;

    println!("Point: ({x}, {y})");
    println!("Coordinates: {} and {}", point.0, point.1);
}
