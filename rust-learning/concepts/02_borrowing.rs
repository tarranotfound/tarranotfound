fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    let message = String::from("small steps in Rust");
    println!("{} words", word_count(&message));
    println!("Still owned here: {message}");
}
