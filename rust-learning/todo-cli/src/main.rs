use std::io::{self, Write};

fn print_help() {
    println!("Commands: add <task>, list, done <number>, help, quit");
}

fn main() {
    let mut tasks: Vec<(String, bool)> = Vec::new();
    println!("Tiny TODO — type help for commands.");

    loop {
        print!("todo> ");
        if io::stdout().flush().is_err() { break; }
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => { eprintln!("Input error: {error}"); break; }
        }
        let line = line.trim();
        if line == "quit" || line == "exit" { break; }
        if line == "help" { print_help(); continue; }
        if line == "list" {
            if tasks.is_empty() { println!("No tasks yet."); }
            for (index, (task, done)) in tasks.iter().enumerate() {
                println!("{}. [{}] {}", index + 1, if *done { "x" } else { " " }, task);
            }
            continue;
        }
        if let Some(task) = line.strip_prefix("add ") {
            let task = task.trim();
            if task.is_empty() { println!("Task cannot be empty."); }
            else { tasks.push((task.to_owned(), false)); println!("Added task {}.", tasks.len()); }
            continue;
        }
        if let Some(number) = line.strip_prefix("done ") {
            match number.trim().parse::<usize>() {
                Ok(index) if index > 0 && index <= tasks.len() => {
                    tasks[index - 1].1 = true;
                    println!("Marked task {index} as done.");
                }
                _ => println!("Use done <valid task number>."),
            }
            continue;
        }
        if !line.is_empty() { println!("Unknown command. Type help."); }
    }
}
