// Demonstrate safe handling of optional values.
fn find_user(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Ada"),
        2 => Some("Grace"),
        _ => None,
    }
}

fn main() {
    let name = find_user(1).unwrap_or("Unknown");
    println!("User: {name}");
}