#[derive(Debug)]
struct Project {
    name: String,
    completed: bool,
}

fn main() {
    let project = Project { name: String::from("tiny-cli"), completed: false };
    println!("{} — completed: {}", project.name, project.completed);
    println!("debug: {project:?}");
}
