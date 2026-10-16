use std::collections::HashMap;

fn main() {
    let sentence = "rust makes small tools fun rust";
    let mut counts = HashMap::new();
    for word in sentence.split_whitespace() {
        *counts.entry(word).or_insert(0usize) += 1;
    }
    for (word, count) in counts {
        println!("{word}: {count}");
    }
}
