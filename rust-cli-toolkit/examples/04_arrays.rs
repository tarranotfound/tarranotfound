fn main() {
    let scores = [7, 9, 8, 10, 6];
    let total: i32 = scores.iter().sum();
    let average = total as f64 / scores.len() as f64;

    println!("Scores: {scores:?}");
    println!("Average: {average:.1}");
}
