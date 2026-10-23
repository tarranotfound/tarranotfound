trait Describe {
    fn describe(&self) -> String;
}

struct Tool {
    name: String,
}

impl Describe for Tool {
    fn describe(&self) -> String {
        format!("{} is ready to use", self.name)
    }
}

fn main() {
    let tool = Tool {
        name: String::from("Rust CLI"),
    };

    println!("{}", tool.describe());
}
