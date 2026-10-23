fn main() {
    let message = String::from("hello");
    print_message(message);
    // message was moved, so it cannot be used here.
}

fn print_message(text: String) {
    println!("{text}");
}
