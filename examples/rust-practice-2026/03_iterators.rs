// Use iterator adapters to transform and filter data.
fn main() {
    let values = [1, 2, 3, 4, 5, 6];
    let squares: Vec<i32> = values
        .iter()
        .filter(|&&value| value % 2 == 0)
        .map(|&value| value * value)
        .collect();

    println!("{squares:?}");
}