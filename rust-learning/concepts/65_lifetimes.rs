fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}

fn main() {
    let first = String::from("ownership");
    let second = String::from("borrowing");

    println!("Longer word: {}", longer(&first, &second));
}