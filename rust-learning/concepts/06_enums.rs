enum Status {
    Draft,
    Published,
    Archived,
}

fn label(status: Status) -> &'static str {
    match status {
        Status::Draft => "draft",
        Status::Published => "published",
        Status::Archived => "archived",
    }
}

fn main() {
    println!("{}", label(Status::Published));
}
