use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        sender.send(String::from("message from worker")).unwrap();
    });
    println!("Received: {}", receiver.recv().unwrap());
}
