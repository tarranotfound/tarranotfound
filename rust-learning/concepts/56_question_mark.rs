use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn main() {
    match read_config("config.txt") {
        Ok(contents) => println!("{contents}"),
        Err(error) => eprintln!("Could not read config: {error}"),
    }
}