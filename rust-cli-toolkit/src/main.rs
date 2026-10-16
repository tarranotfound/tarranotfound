mod calculator;
mod converter;
mod history;
mod menu;
mod text_tools;
mod validation;

use std::io::{self, Write};

fn main() {
    let mut history = history::History::new();

    loop {
        menu::print_menu();
        print!("Choose an option: ");
        let _ = io::stdout().flush();

        let choice = match validation::read_line() {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Input error: {error}");
                continue;
            }
        };

        match choice.trim() {
            "1" => calculator::run(&mut history),
            "2" => converter::run(),
            "3" => text_tools::run(),
            "4" => history.print(),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown option. Choose 0-4."),
        }

        println!();
    }
}
