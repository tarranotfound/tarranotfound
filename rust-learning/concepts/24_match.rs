fn describe(value: i32) -> &'static str {
    match value {
        0 => "zero",
        1..=9 => "one digit",
        10..=99 => "two digits",
        _ if value < 0 => "negative",
        _ => "larger number",
    }
}

fn main() {
    for value in [-5, 0, 7, 42, 120] {
        println!("{value}: {}", describe(value));
    }
}
