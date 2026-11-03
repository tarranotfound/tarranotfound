use std::thread;

fn main() {
    let values = [1, 2, 3, 4];

    // Scoped threads may safely borrow values from the parent scope.
    let total = thread::scope(|scope| {
        let worker = scope.spawn(|| values.iter().sum::<i32>());
        worker.join().expect("worker thread panicked")
    });

    println!("total: {total}");
}
