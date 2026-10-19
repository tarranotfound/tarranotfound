fn main() {
    let numbers = vec![3, 10, 4, 15, 8, 21];
    let even_numbers: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|number| number % 2 == 0)
        .collect();

    println!("Numbers: {numbers:?}");
    println!("Even numbers: {even_numbers:?}");
}
