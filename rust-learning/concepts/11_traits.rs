trait Summary {
    fn summary(&self) -> String;
}

struct Note { title: String }

impl Summary for Note {
    fn summary(&self) -> String {
        format!("Note: {}", self.title)
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn main() {
    print_summary(&Note { title: String::from("Learning Rust") });
}
