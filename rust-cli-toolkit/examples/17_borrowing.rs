fn main() {
    let mut score = 10;

    add_bonus(&mut score);
    show_score(&score);
}

fn add_bonus(score: &mut i32) {
    *score += 5;
}

fn show_score(score: &i32) {
    println!("Score: {score}");
}
