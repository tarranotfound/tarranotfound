fn main() {
    let values = [8, 3, 14, 6, 11, 2];
    let min = values.iter().min().expect("array is not empty");
    let max = values.iter().max().expect("array is not empty");
    println!("values: {values:?}");
    println!("range: {min}..={max}");
}
