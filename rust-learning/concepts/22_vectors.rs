fn main() {
    let mut numbers = vec![10, 20, 30];
    numbers.push(40);

    if let Some(first) = numbers.first() {
        println!("first: {first}");
    }

    for number in &numbers {
        println!("{number}");
    }

    println!("count: {}", numbers.len());
}
