use std::collections::HashMap;

fn main() {
    let words = ["rust", "cli", "rust", "tools", "cli", "rust"];
    let mut counts = HashMap::new();

    for word in words {
        *counts.entry(word).or_insert(0) += 1;
    }

    println!("Word counts: {counts:?}");
}
