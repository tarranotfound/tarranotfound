use std::rc::Rc;

fn main() {
    let original = Rc::new(String::from("shared data"));
    let first = Rc::clone(&original);
    let second = Rc::clone(&original);

    println!("value: {first}");
    println!("owners: {}", Rc::strong_count(&second));
}
