fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

fn main() {
    let sentence = String::from("Rust makes systems programming fun");
    println!("First word: {}", first_word(&sentence));
}
