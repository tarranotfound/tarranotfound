// Trait objects enable runtime polymorphism.
trait Draw { fn draw(&self); }
struct Circle;
struct Square;
impl Draw for Circle { fn draw(&self) { println!("circle"); } }
impl Draw for Square { fn draw(&self) { println!("square"); } }

fn render(item: &dyn Draw) { item.draw(); }
fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![Box::new(Circle), Box::new(Square)];
    for shape in &shapes { render(shape.as_ref()); }
}
