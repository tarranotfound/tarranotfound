fn parse_port(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse::<u16>()
}

fn main() {
    let fallback = Some(8080).unwrap_or(3000);
    println!("fallback port: {fallback}");

    match parse_port("9000") {
        Ok(port) => println!("port: {port}"),
        Err(error) => eprintln!("invalid port: {error}"),
    }
}
