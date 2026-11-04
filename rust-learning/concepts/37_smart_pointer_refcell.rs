use std::cell::RefCell;

fn main() {
    let score = RefCell::new(10);

    *score.borrow_mut() += 5;
    let current = score.borrow();

    println!("score: {current}");
}
