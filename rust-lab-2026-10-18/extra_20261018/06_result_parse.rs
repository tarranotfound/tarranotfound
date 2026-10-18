use std::num::ParseIntError;

fn parse_port(input: &str) -> Result<u16, ParseIntError> {
    input.parse::<u16>()
}

fn main() {
    for candidate in ["8080", "65535", "70000", "web"] {
        match parse_port(candidate) {
            Ok(port) => println!("{candidate} -> valid port {port}"),
            Err(error) => println!("{candidate} -> invalid port ({error})"),
        }
    }
}
