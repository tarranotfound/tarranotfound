fn main() {
    let multiplier = 3;
    let scale = |value: i32| value * multiplier;
    let values: Vec<i32> = (1..=5).map(scale).collect();
    println!("{values:?}");
}
