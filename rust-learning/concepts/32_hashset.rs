use std::collections::HashSet;

fn main() {
    let mut languages = HashSet::new();
    languages.insert("Rust");
    languages.insert("C++");
    languages.insert("Rust"); // Duplicate values are ignored.

    println!("contains Rust: {}", languages.contains("Rust"));
    println!("unique languages: {}", languages.len());

    for language in &languages {
        println!("{language}");
    }
}
