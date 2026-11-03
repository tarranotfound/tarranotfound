mod library {
    pub mod books {
        #[derive(Debug)]
        pub struct Book {
            pub title: String,
        }

        impl Book {
            pub fn new(title: &str) -> Self {
                Self {
                    title: title.to_owned(),
                }
            }
        }
    }
}

use library::books::Book;

fn main() {
    let book = Book::new("The Rust Programming Language");
    println!("{:?}", book);
}
