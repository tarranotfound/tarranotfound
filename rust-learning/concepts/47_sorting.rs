fn main() {
    let mut names = vec!["Mira", "Akhtar", "Bima"];
    names.sort();
    println!("Alphabetical: {names:?}");

    let mut numbers = vec![8, 2, 5, 1];
    numbers.sort_unstable();
    println!("Numbers: {numbers:?}");
}
