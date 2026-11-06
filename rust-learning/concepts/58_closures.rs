fn main() {
    let multiplier = 3;
    let scale = |value: i32| value * multiplier;

    for value in 1..=5 {
        println!("{value} x {multiplier} = {}", scale(value));
    }
}