use std::path::Path;

fn main() {
    for name in ["main.rs", "notes.txt", "README", "config.toml"] {
        let extension = Path::new(name).extension().and_then(|ext| ext.to_str()).unwrap_or("(none)");
        println!("{name} -> {extension}");
    }
}
