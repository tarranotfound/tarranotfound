// Mutability is explicit in Rust bindings.
fn main() {
    let mut count = 0;
    for _ in 0..5 {
        count += 1;
    }

    println!("Count: {count}");
}