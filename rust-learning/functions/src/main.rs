fn square(value: i32) -> i32 {
    value * value
}

fn describe_score(score: u8) -> &'static str {
    if score >= 90 { "excellent" } else if score >= 75 { "good" } else { "keep practicing" }
}

fn main() {
    let value = 7;
    println!("{value} squared is {}", square(value));
    println!("Assessment: {}", describe_score(88));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_multiplies_value_by_itself() { assert_eq!(square(7), 49); }

    #[test]
    fn score_description_uses_thresholds() {
        assert_eq!(describe_score(90), "excellent");
        assert_eq!(describe_score(75), "good");
        assert_eq!(describe_score(40), "keep practicing");
    }
}
