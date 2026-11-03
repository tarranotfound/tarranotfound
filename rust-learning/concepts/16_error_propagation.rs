use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    // The ? operator returns early when an operation fails.
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn main() {
    match read_config("config.txt") {
        Ok(contents) => println!("{contents}"),
        Err(error) => eprintln!("could not read config: {error}"),
    }
}
