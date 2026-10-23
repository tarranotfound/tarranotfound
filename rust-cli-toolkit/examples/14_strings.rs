fn main() {
    let mut name = String::from("Rust");
    name.push_str(" CLI");

    let language: &str = &name;
    println!("Learning {language}");
    println!("Length: {} bytes", name.len());
}
