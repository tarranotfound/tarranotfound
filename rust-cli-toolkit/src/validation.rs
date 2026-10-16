use std::io::{self, Write};

pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn read_line_prompt(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    read_line()
}

pub fn read_number(prompt: &str) -> Result<f64, String> {
    let input = read_line_prompt(prompt).map_err(|error| error.to_string())?;
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| "Please enter a valid number.".to_string())
}
