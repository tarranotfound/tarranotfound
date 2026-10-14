use std::io::{self, Write};

fn main() {
    let secret = 7_u32;
    println!("Guess the number (1-10). Type q to quit.");

    loop {
        print!("guess> ");
        if io::stdout().flush().is_err() {
            eprintln!("Could not flush terminal output.");
            break;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                eprintln!("Could not read input: {error}");
                break;
            }
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") { break; }
        match input.parse::<u32>() {
            Ok(guess) if !(1..=10).contains(&guess) => println!("Choose a number from 1 to 10."),
            Ok(guess) if guess < secret => println!("Too low."),
            Ok(guess) if guess > secret => println!("Too high."),
            Ok(_) => { println!("Correct!"); break; }
            Err(_) => println!("Enter a valid whole number, or q to quit."),
        }
    }
}
