use std::fs;
use std::io;

fn save_note(path: &str, note: &str) -> io::Result<()> {
    fs::write(path, note)
}

fn main() -> io::Result<()> {
    save_note("note.txt", "Learning Rust file I/O")?;

    let contents = fs::read_to_string("note.txt")?;
    println!("{contents}");
    Ok(())
}
