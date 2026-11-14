// Enums model values that can be one of several variants.
enum Command {
    Start,
    Stop,
    SetLevel(u8),
}

fn describe(command: Command) -> &'static str {
    match command {
        Command::Start => "starting",
        Command::Stop => "stopping",
        Command::SetLevel(level) if level > 100 => "level out of range",
        Command::SetLevel(_) => "updating level",
    }
}

fn main() {
    println!("{}", describe(Command::SetLevel(42)));
}