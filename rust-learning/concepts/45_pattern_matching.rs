fn describe(value: (i32, i32)) -> &'static str {
    match value {
        (0, 0) => "origin",
        (0, _) => "on the y axis",
        (_, 0) => "on the x axis",
        (x, y) if x == y => "on the diagonal",
        _ => "somewhere else",
    }
}

fn main() { println!("{}", describe((3, 3))); }
