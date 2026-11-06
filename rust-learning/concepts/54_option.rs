fn main() {
    let username = Some("tarranotfound");
    let guest: Option<&str> = None;

    println!("User: {}", username.unwrap_or("anonymous"));
    println!("Guest: {}", guest.unwrap_or("anonymous"));
}