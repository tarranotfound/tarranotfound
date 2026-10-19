fn first_character(text: &str) -> Option<char> {
    text.chars().next()
}

fn main() {
    for text in ["rust", ""] {
        match first_character(text) {
            Some(character) => println!("First character: {character}"),
            None => println!("The string is empty"),
        }
    }
}
