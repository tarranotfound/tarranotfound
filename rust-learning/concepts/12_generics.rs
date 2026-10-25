fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut result = *items.first()?;
    for &item in &items[1..] {
        if item > result { result = item; }
    }
    Some(result)
}

fn main() {
    println!("{:?}", largest(&[4, 9, 2, 7]));
}
