// Slices borrow a contiguous part of a collection.
fn total(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let numbers = [4, 8, 15, 16, 23, 42];
    println!("Total: {}", total(&numbers[1..4]));
}