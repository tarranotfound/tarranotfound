use std::collections::HashSet;

fn main() {
    let tags = ["rust", "linux", "rust", "systems"];
    let unique: HashSet<&str> = tags.into_iter().collect();

    println!("Unique tags: {}", unique.len());
    for tag in &unique {
        println!("- {tag}");
    }
}