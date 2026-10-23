use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = "example-note.txt";
    fs::write(path, "Learning Rust file I/O")?;

    let contents = fs::read_to_string(path)?;
    println!("{contents}");

    fs::remove_file(path)?;
    Ok(())
}
