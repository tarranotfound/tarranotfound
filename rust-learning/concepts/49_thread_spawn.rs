use std::thread;

fn main() {
    let worker = thread::spawn(|| {
        (1..=5).sum::<u32>()
    });
    let total = worker.join().expect("worker thread panicked");
    println!("Sum from worker: {total}");
}
