use crate::validation;

pub fn reverse_text(text: &str) -> String {
    text.chars().rev().collect()
}

pub fn run() {
    let text = match validation::read_line_prompt("Enter text: ") {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    let trimmed = text.trim_end();

    println!("Reversed: {}", reverse_text(trimmed));
    println!("Characters: {}", trimmed.chars().count());
    println!("Words: {}", trimmed.split_whitespace().count());
}
