fn parse_port(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse::<u16>()
}

fn main() {
    let port = parse_port("8080")
        .map(|value| value + 1)
        .unwrap_or(3000);

    println!("port: {port}");

    match parse_port("not-a-port") {
        Ok(value) => println!("parsed: {value}"),
        Err(error) => eprintln!("invalid port: {error}"),
    }
}
