fn describe_number(number: i32) -> &'static str {
    match number {
        0 => "zero",
        1..=9 => "single digit",
        _ if number < 0 => "negative",
        _ => "multiple digits",
    }
}

fn main() {
    for number in [-3, 0, 4, 12] {
        println!("{number}: {}", describe_number(number));
    }
}
