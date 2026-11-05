use std::sync::Arc;
use std::thread;

fn main() {
    let shared = Arc::new(vec![2, 4, 6, 8]);
    let handles: Vec<_> = (0..2).map(|_| {
        let data = Arc::clone(&shared);
        thread::spawn(move || data.iter().sum::<i32>())
    }).collect();
    for handle in handles { println!("Worker sum: {}", handle.join().unwrap()); }
}
