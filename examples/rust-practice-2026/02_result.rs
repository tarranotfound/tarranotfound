// Demonstrate recoverable errors with Result.
use std::num::ParseIntError;

fn parse_age(input: &str) -> Result<u8, ParseIntError> {
    input.parse()
}

fn main() {
    match parse_age("17") {
        Ok(age) => println!("Age: {age}"),
        Err(error) => eprintln!("Invalid age: {error}"),
    }
}