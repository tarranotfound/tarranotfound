fn append_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut message = String::from("hello");
    append_exclamation(&mut message);
    println!("{message}");
}
