use std::env;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| String::from("app"));

    let Some(name) = args.next() else {
        eprintln!("usage: {program} <name>");
        return;
    };

    println!("hello, {name}");
}

// Run with: cargo run -- Ada
