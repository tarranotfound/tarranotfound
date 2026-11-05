use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.trim().to_owned())
}

fn main() {
    match read_config("config.txt") {
        Ok(value) => println!("Config: {value}"),
        Err(error) => eprintln!("Could not read config: {error}"),
    }
}
