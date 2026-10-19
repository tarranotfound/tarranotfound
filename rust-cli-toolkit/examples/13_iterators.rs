fn main() {
    let total: i32 = (1..=10)
        .filter(|number| number % 2 == 0)
        .map(|number| number * number)
        .sum();

    println!("Sum of squares of even numbers from 1 to 10: {total}");
}
