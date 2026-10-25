fn main() {
    let squares: Vec<u32> = (1..=8)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();
    println!("{squares:?}");
}
