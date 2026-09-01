mod build_ins;
use crate::build_ins::check::BuildInCommand;
use build_ins::cd;
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
        let commands = command.split_whitespace().collect::<Vec<&str>>();
        let command_type = BuildInCommand::from(commands[0]);

        match command_type {
            BuildInCommand::Exit => break,
            BuildInCommand::Cd => {
                let path = command[3..].trim();
                if cd(path).is_err() {
                    eprintln!("Failed to change directory");
                }
                continue;
            }
            BuildInCommand::Other(cmd) => match Command::new(cmd).args(&commands[1..]).status() {
                Ok(_) => (),
                Err(_) => {
                    if cd(commands[0]).is_err() {
                        eprintln!("Command not found");
                    }
                }
            },
        }
    }
}
