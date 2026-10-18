fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0_u64, 1_u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

fn main() {
    for n in 0..=15 {
        println!("F({n}) = {}", fibonacci(n));
    }
}
