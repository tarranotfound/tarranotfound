// Define shared behavior with a trait.
trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let article = Article {
        title: "Learning Rust".into(),
        author: "Tarra".into(),
    };
    println!("{}", article.summary());
}