fn parse_score(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse()
}

fn main() {
    for input in ["88", "oops"] {
        match parse_score(input) {
            Ok(score) => println!("Parsed score: {score}"),
            Err(error) => println!("Could not parse score: {error}"),
        }
    }
}
