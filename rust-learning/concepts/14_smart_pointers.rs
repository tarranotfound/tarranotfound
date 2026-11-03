// Smart pointers provide ownership-aware ways to manage data.
// Box<T> stores a value on the heap while keeping a single owner.

fn main() {
    let number = Box::new(42);
    println!("boxed value: {number}");

    // Box also makes recursive data structures possible.
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    let mut current = &list;
    loop {
        match current {
            Cons(value, next) => {
                println!("{value}");
                current = next;
            }
            Nil => break,
        }
    }
}
