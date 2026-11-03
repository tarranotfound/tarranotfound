use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("green"), 20);

    // Entry inserts a default only when the key is absent.
    scores.entry(String::from("blue")).or_insert(15);
    scores.entry(String::from("red")).or_insert(5);

    if let Some(score) = scores.get("green") {
        println!("green: {score}");
    }

    for (team, score) in &scores {
        println!("{team}: {score}");
    }
}
