fn main() {
    let multiplier = 3;
    let multiply = |number: i32| number * multiplier;

    println!("4 times {multiplier} = {}", multiply(4));

    let values = [1, 2, 3, 4];
    let doubled: Vec<i32> = values.iter().map(|value| value * 2).collect();
    println!("Doubled values: {doubled:?}");
}
