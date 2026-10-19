enum Command {
    Add(i32, i32),
    Repeat(String),
    Quit,
}

fn run(command: Command) {
    match command {
        Command::Add(left, right) => println!("Result: {}", left + right),
        Command::Repeat(message) => println!("{message}"),
        Command::Quit => println!("Goodbye"),
    }
}

fn main() {
    run(Command::Add(12, 30));
    run(Command::Repeat(String::from("Hello from Rust")));
    run(Command::Quit);
}
