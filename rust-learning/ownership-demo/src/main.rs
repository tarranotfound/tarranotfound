fn character_count(text: &str) -> usize {
    text.chars().count()
}

fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

fn main() {
    let mut message = String::from("Learn");
    append_suffix(&mut message, " Rust");
    println!("{message} ({} characters)", character_count(&message));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_unicode_characters() {
        assert_eq!(character_count("hé"), 2);
    }

    #[test]
    fn mutable_borrow_appends_suffix() {
        let mut text = String::from("Hello");
        append_suffix(&mut text, "!");
        assert_eq!(text, "Hello!");
    }
}
