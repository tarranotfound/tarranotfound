use std::thread;
use std::time::Duration;

fn main() {
    let pause = Duration::from_millis(250);
    println!("Starting short task...");
    thread::sleep(pause);
    println!("Task complete.");
}
