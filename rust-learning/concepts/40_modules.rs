// A small example of organizing code into modules.
mod geometry {
    pub fn rectangle_area(width: f64, height: f64) -> f64 {
        width * height
    }
}

fn main() {
    let area = geometry::rectangle_area(4.0, 2.5);
    println!("Area: {area}");
}
