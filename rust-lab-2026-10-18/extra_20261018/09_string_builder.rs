use std::fmt::Write;

fn main() {
    let items = ["cargo", "rustc", "clippy"];
    let mut report = String::from("Rust toolchain checklist\n");

    for (index, item) in items.iter().enumerate() {
        writeln!(&mut report, "{}. [x] {item}", index + 1)
            .expect("writing to a String cannot fail");
    }

    print!("{report}");
}
