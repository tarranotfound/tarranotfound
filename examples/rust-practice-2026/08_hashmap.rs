// HashMap stores values associated with keys.
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("green"), 25);

    if let Some(score) = scores.get("green") {
        println!("Green team: {score}");
    }
}