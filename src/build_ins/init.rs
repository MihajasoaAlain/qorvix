use std::io::{self, Write};

use crate::build_ins::check::BuildInCommand;
use crate::build_ins::{cd, parser, pipeline};

pub fn intro(greetings: &str, info: &str) {
    println!("{}\n{}", greetings, info);
}

pub fn execute_command() {
    loop {
        let input = match user_command() {
            Some(input) => input,
            None => break, };

        let parsed_command = match parser::parse(input) {
            Some(cmd) => cmd,
            None => continue,
        };
        if parsed_command.len() == 1 {
            if execute(parsed_command[0].clone()) {
                break;
            }
        } else {
            pipeline::execute_pipeline(&parsed_command);
        }
    }
}
pub fn execute(parsed_command: parser::ParsedCommand) -> bool {
    match parsed_command.program {
        BuildInCommand::Exit => true,
        BuildInCommand::Cd => {
            if parsed_command.arguments.len() > 1 {
                eprintln!("qorvix: {}: too many arguments", parsed_command.program);
                return false;
            }
            let path = match parsed_command.arguments.first() {
                Some(path) => path.as_str(),
                None => "/",
            };
            if let Err(error) = cd(path) {
                eprintln!("qorvix: {}: {}: {}", parsed_command.program, path, error);
            }
            false
        }
        BuildInCommand::Other(_) => {
            pipeline::execute_pipeline(std::slice::from_ref(&parsed_command));
            false
        }
    }
}

pub fn user_command() -> Option<String> {
    let current_dir = std::env::current_dir().unwrap_or_default();
    print!("{} > ", current_dir.display());
    let _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => {
            println!();
            None
        }
        Ok(_) => Some(input.trim().to_string()),
        Err(error) => {
            eprintln!("qorvix: {}", error);
            None
        }
    }
}
