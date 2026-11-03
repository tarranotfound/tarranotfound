fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];

    let doubled_even: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|number| number % 2 == 0)
        .map(|number| number * 2)
        .collect();

    println!("{doubled_even:?}");
}
