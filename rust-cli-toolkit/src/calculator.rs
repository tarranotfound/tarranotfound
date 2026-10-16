use crate::{history::History, validation};

pub fn calculate(a: f64, operator: char, b: f64) -> Result<f64, String> {
    match operator {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' if b != 0.0 => Ok(a / b),
        '/' => Err("Cannot divide by zero.".into()),
        _ => Err(format!("Unsupported operator: {operator}")),
    }
}

pub fn run(history: &mut History) {
    println!("Calculator — enter two numbers and an operator (+ - * /).");
    let a = match validation::read_number("First number: ") {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    let operator_text = match validation::read_line_prompt("Operator: ") {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    let Some(operator) = operator_text.trim().chars().next() else {
        println!("Please enter an operator.");
        return;
    };
    let b = match validation::read_number("Second number: ") {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    match calculate(a, operator, b) {
        Ok(result) => {
            println!("Result: {result}");
            history.push(format!("{a} {operator} {b} = {result}"));
        }
        Err(error) => println!("{error}"),
    }
}
