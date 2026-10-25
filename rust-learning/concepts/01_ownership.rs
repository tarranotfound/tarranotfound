// Ownership moves a String unless it is explicitly cloned.
fn main() {
    let original = String::from("blankspace");
    let moved = original;
    println!("{moved}");
    // `original` is no longer usable here because its value was moved.
}
