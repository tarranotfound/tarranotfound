fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(add(-2, 1), -1);
    }
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
}

// Run tests with: rustc --test 18_unit_tests.rs && ./18_unit_tests
