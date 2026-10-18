fn first_long_word(words: &[&str], min_len: usize) -> Option<&str> {
    words.iter().copied().find(|word| word.len() >= min_len)
}

fn main() {
    let words = ["cli", "parser", "rust", "terminal"];
    match first_long_word(&words, 7) {
        Some(word) => println!("Found: {word}"),
        None => println!("No word matched"),
    }
}
