use std::fmt;

#[derive(Debug)]
enum ParseAgeError { Empty, InvalidNumber }
impl fmt::Display for ParseAgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "age is empty"),
            Self::InvalidNumber => write!(f, "age is not a valid number"),
        }
    }
}

fn parse_age(input: &str) -> Result<u8, ParseAgeError> {
    if input.trim().is_empty() { return Err(ParseAgeError::Empty); }
    input.trim().parse().map_err(|_| ParseAgeError::InvalidNumber)
}

fn main() { println!("{:?}", parse_age("17")); }
