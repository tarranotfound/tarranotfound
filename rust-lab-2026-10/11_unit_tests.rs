fn clamp(value: i32, low: i32, high: i32) -> i32 {
    value.max(low).min(high)
}

fn main() {
    println!("clamp(12, 0, 10) = {}", clamp(12, 0, 10));
}

#[cfg(test)]
mod tests {
    use super::clamp;
    #[test] fn keeps_values_in_range() { assert_eq!(clamp(5, 0, 10), 5); }
    #[test] fn caps_high_values() { assert_eq!(clamp(12, 0, 10), 10); }
    #[test] fn raises_low_values() { assert_eq!(clamp(-2, 0, 10), 0); }
}
