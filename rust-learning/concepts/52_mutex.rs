use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let workers: Vec<_> = (0..4).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            *value += 1;
        })
    }).collect();
    for worker in workers { worker.join().unwrap(); }
    println!("Counter: {}", *counter.lock().unwrap());
}
