#[derive(Debug)]
struct Project {
    name: String,
    stars: u32,
}

fn main() {
    let project = Project { name: String::from("blankspace"), stars: 42 };
    println!("{} has {} stars", project.name, project.stars);
    println!("{project:?}");
}
