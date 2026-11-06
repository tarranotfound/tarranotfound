use std::collections::VecDeque;

fn main() {
    let mut tasks = VecDeque::new();
    tasks.push_back("compile");
    tasks.push_back("test");
    tasks.push_front("format");

    while let Some(task) = tasks.pop_front() {
        println!("Running task: {task}");
    }
}