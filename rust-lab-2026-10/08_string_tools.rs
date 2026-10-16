fn main() {
    let input = "  Build tiny tools with Rust  ";
    let cleaned = input.trim();
    println!("trimmed: {cleaned}");
    println!("characters: {}", cleaned.chars().count());
    println!("uppercase: {}", cleaned.to_uppercase());
}
