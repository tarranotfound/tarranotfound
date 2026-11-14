// String owns UTF-8 text and can grow dynamically.
fn main() {
    let mut message = String::from("Hello");
    message.push(',');
    message.push_str(" Rust");

    let decorated = format!("{message}!");
    println!("{decorated}");
}