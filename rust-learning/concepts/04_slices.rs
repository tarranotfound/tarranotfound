fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("borrowed string slices");
    println!("{}", first_word(&sentence));
}
