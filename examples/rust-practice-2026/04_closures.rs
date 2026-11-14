// Closures can capture values from their environment.
fn main() {
    let multiplier = 3;
    let scale = |value: i32| value * multiplier;

    println!("{}", scale(7));
}