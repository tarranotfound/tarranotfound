// A generic function works with multiple concrete types.
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    let numbers = [10, 20, 30];
    let words = ["rust", "systems"];

    println!("{:?}", first(&numbers));
    println!("{:?}", first(&words));
}