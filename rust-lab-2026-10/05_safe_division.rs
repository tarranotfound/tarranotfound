fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    for (a, b) in [(12.0, 3.0), (8.0, 0.0)] {
        match divide(a, b) {
            Some(value) => println!("{a} / {b} = {value}"),
            None => println!("cannot divide {a} by zero"),
        }
    }
}
