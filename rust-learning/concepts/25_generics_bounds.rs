use std::fmt::Display;

fn print_pair<T: Display, U: Display>(left: T, right: U) {
    println!("({left}, {right})");
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut result = *items.first()?;
    for &item in &items[1..] {
        if item > result {
            result = item;
        }
    }
    Some(result)
}

fn main() {
    print_pair("answer", 42);
    println!("largest: {:?}", largest(&[3, 9, 2, 7]));
}
