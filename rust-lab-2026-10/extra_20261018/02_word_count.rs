use std::collections::HashMap;

fn main() {
    let text = "rust makes systems programming fun and rust makes tools";
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0usize) += 1;
    }

    let mut words: Vec<_> = counts.iter().collect();
    words.sort_by_key(|(word, _)| *word);
    for (word, count) in words {
        println!("{word}: {count}");
    }
}
