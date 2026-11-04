fn main() {
    let mut message = String::from("hello");
    message.push(' ');
    message.push_str("Rust");

    let language = "Rust";
    println!("{message} has {} characters", language.len());

    for character in message.chars() {
        print!("{character}-");
    }
    println!();
}
