fn main() {
    let first = String::from("borrow checker");
    let second = String::from("Rust");

    let longer = longest(first.as_str(), second.as_str());
    println!("Longer text: {longer}");
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
