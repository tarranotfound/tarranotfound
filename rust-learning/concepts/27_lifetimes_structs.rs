#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn preview(&self, limit: usize) -> &str {
        let end = self.text.char_indices()
            .nth(limit)
            .map(|(index, _)| index)
            .unwrap_or(self.text.len());
        &self.text[..end]
    }
}

fn main() {
    let article = String::from("Ownership and borrowing");
    let excerpt = Excerpt { text: &article };
    println!("{:?}", excerpt);
    println!("{}", excerpt.preview(9));
}
