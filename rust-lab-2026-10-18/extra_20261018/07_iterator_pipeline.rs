fn main() {
    let values = 1..=20;
    let squares: Vec<u32> = values
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .take(5)
        .collect();

    println!("First five even squares: {squares:?}");
}
