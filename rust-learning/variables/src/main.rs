fn main() {
    let immutable_value = 10;
    let mut counter = 0;
    counter += immutable_value;

    let label = if counter > 5 { "greater than five" } else { "five or less" };
    println!("counter = {counter} ({label})");
}
