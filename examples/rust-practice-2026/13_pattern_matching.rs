// Pattern matching makes branching over structured data concise.
fn classify(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origin",
        (0, _) => "y-axis",
        (_, 0) => "x-axis",
        (x, y) if x == y => "diagonal",
        _ => "other",
    }
}

fn main() {
    println!("{}", classify((3, 3)));
}