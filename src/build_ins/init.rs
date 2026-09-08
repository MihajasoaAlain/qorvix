use std::io::{self, Write};
use std::process::Command;

use crate::build_ins::check::BuildInCommand;
use crate::build_ins::{cd, parser};

pub fn intro(greetings: &str, info: &str) {
    println!("{}\n{}", greetings, info);
}

pub fn execute_command() {
    loop {
        let input = user_command();

        let parsed_command = match parser::parse(input) {
            Some(cmd) => cmd,
            None => continue,
        };
        println!("Parsed command: {:?}", parsed_command);
        if parsed_command.len() == 1 {
            if execute(parsed_command[0].clone()) {
                break;
            } else {
                continue;
            }
        } else {
            parser::execute_pipeline(&parsed_command);
        }
    }
}
pub fn execute(parsed_command: parser::ParsedCommand) -> bool {
    match parsed_command.program {
        BuildInCommand::Exit => true,
        BuildInCommand::Cd => {
            if parsed_command.arguments.len() > 2 {
                eprintln!(
                    "{:?}: string not in pwd: {}",
                    parsed_command.program,
                    parsed_command.arguments.join(" ")
                );
            }
            if parsed_command.arguments.is_empty() {
                if cd("/").is_err() {
                    eprintln!("{}: failed to change directory", parsed_command.program);
                }
                return false;
            }
            let path = if !parsed_command.arguments.is_empty() {
                parsed_command.arguments[0].clone()
            } else {
                "/".into()
            };
            if cd(path.as_str()).is_err() {
                eprintln!("{}: failed to change directory", parsed_command.program);
            }
            false
        }
        BuildInCommand::Other(ref cmd) => {
            match  Command::new(cmd).args(&parsed_command.arguments).status() {
                Ok(_) => false,
                Err(_) => {
                    eprintln!("{}: command not found", parsed_command.program);
                    false
                }
            }
        }
    }
}

pub fn user_command() -> String {
    let current_dir = Command::new("pwd").output().unwrap().stdout;
    print!("{} > ", String::from_utf8(current_dir).unwrap().trim());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
