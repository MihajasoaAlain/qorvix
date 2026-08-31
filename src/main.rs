use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Qorvix Shell");

    println!("Type 'exit' to quit the shell.");

    loop {
        let current_dir = Command::new("pwd").output().unwrap().stdout;
        print!("{} > ", String::from_utf8(current_dir).unwrap().trim());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command == "exit" {
            break;
        }
        if command.starts_with("cd ") {
            let path = command[3..].trim();
            if let Err(e) = std::env::set_current_dir(path) {
                eprintln!("qorvix: cd: {}: {}", path, e);
            }
            continue;
        }
        let commands = command.split_whitespace().collect::<Vec<&str>>();
        match Command::new(commands[0]).args(&commands[1..]).status() {
            Ok(_) => (),
            Err(_) => {
                if let Err(_) = std::env::set_current_dir(commands[0]) {
                    eprintln!("qorvix: Command not found: {}", commands[0]);
                }
            }
        }
    }
}
