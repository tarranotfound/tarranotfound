fn main() {
    let values = [2, 4, 6, 8, 10];

    let doubled: Vec<i32> = values.iter().map(|value| value * 2).collect();
    let total: i32 = doubled.iter().sum();

    println!("Doubled: {doubled:?}");
    println!("Total: {total}");
}