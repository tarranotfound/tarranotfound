#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    let copy = origin.clone();

    println!("{origin:?}");
    assert_eq!(origin, copy);
}
