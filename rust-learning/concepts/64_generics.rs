fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    let numbers = [10, 20, 30];
    let names = ["Ada", "Grace", "Linus"];

    println!("First number: {:?}", first(&numbers));
    println!("First name: {:?}", first(&names));
}