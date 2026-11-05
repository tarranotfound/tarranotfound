#[derive(Debug)]
struct User { name: String, active: bool, score: u32 }

fn main() {
    let original = User { name: "Tarra".into(), active: true, score: 10 };
    let improved = User { score: 15, ..original };
    println!("{improved:?}");
}
