fn main() {
    let numbers: Vec<i32> = (1..=50).filter(|n| n % 2 == 0).collect();
    let sum: i32 = numbers.iter().sum();
    println!("evens: {numbers:?}");
    println!("count: {}, sum: {sum}", numbers.len());
}
