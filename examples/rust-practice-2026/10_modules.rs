// A small module keeps related functionality together.
mod math {
    pub fn double(value: i32) -> i32 {
        value * 2
    }
}

fn main() {
    println!("{}", math::double(21));
}