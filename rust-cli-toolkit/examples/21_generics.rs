fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    let numbers = [8, 13, 21];
    let words = ["learn", "build"];

    println!("First number: {:?}", first(&numbers));
    println!("First word: {:?}", first(&words));
}
