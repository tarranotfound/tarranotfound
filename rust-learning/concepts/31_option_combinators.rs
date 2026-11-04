fn username(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("ada"),
        2 => Some("grace"),
        _ => None,
    }
}

fn main() {
    let display_name = username(1)
        .map(str::to_uppercase)
        .unwrap_or_else(|| String::from("GUEST"));

    let is_known = username(3).is_some();
    println!("{display_name}; known: {is_known}");
}
