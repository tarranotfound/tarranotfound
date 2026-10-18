#[derive(Debug)]
enum TaskStatus {
    Todo,
    InProgress { percent: u8 },
    Done,
}

fn label(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Todo => "waiting",
        TaskStatus::InProgress { .. } => "active",
        TaskStatus::Done => "complete",
    }
}

fn main() {
    let tasks = [
        TaskStatus::Todo,
        TaskStatus::InProgress { percent: 60 },
        TaskStatus::Done,
    ];
    for task in &tasks {
        println!("{task:?}: {}", label(task));
    }
}
