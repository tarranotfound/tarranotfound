#[derive(Debug)]
enum Status { Todo, InProgress, Done }

fn main() {
    let tasks = [Status::Todo, Status::InProgress, Status::Done];
    for (index, status) in tasks.iter().enumerate() {
        println!("task {}: {status:?}", index + 1);
    }
}
