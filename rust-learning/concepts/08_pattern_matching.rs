fn describe(value: Option<i32>) -> &'static str {
    match value {
        Some(0) => "zero",
        Some(n) if n > 0 => "positive",
        Some(_) => "negative",
        None => "missing",
    }
}

fn main() {
    for value in [Some(3), Some(0), Some(-2), None] {
        println!("{}", describe(value));
    }
}
