use std::num::ParseIntError;

fn parse_score(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse()
}

fn main() {
    match parse_score("42") {
        Ok(score) => println!("Score: {score}"),
        Err(error) => eprintln!("Invalid score: {error}"),
    }
}