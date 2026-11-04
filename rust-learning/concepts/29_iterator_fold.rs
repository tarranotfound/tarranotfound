fn main() {
    let values = [2, 4, 6, 8];

    let sum = values.iter().fold(0, |total, value| total + value);
    let product = values.iter().copied().product::<i32>();

    println!("sum: {sum}");
    println!("product: {product}");
}
