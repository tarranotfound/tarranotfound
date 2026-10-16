#[derive(Debug, Default)]
pub struct History {
    entries: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn print(&self) {
        if self.entries.is_empty() {
            println!("No calculations in this session yet.");
            return;
        }

        println!("Session history:");
        for (index, entry) in self.entries.iter().enumerate() {
            println!("{}. {entry}", index + 1);
        }
    }
}
