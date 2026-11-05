fn main() {
    let mut scores = vec![12, 18, 20];
    for score in scores.iter_mut() {
        *score += 5;
    }
    println!("Updated scores: {scores:?}");
}
