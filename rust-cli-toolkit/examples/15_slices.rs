fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4];

    println!("Middle slice: {middle:?}");
    println!("First item: {}", middle[0]);
}
