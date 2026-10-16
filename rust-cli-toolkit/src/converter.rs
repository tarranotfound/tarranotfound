use crate::validation;

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

pub fn run() {
    println!("Temperature converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    let choice = match validation::read_line_prompt("Choose conversion: ") {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    let temperature = match validation::read_number("Temperature: ") {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    match choice.trim() {
        "1" => println!("{temperature:.2} °C = {:.2} °F", celsius_to_fahrenheit(temperature)),
        "2" => println!("{temperature:.2} °F = {:.2} °C", fahrenheit_to_celsius(temperature)),
        _ => println!("Unknown conversion."),
    }
}
