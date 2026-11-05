use std::fmt::Write;

fn main() {
    let parts = ["systems", "programming", "with", "Rust"];
    let mut sentence = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 { sentence.push(' '); }
        write!(&mut sentence, "{part}").expect("writing to String cannot fail");
    }
    println!("{sentence}");
}
