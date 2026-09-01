use std::io::{self, Write};
use std::process::Command;

use crate::build_ins::cd;
use crate::build_ins::check::BuildInCommand;

pub fn intro(greetings: &str, info: &str) {
    println!("{}\n{}", greetings, info);
}

pub fn execute_command() {
    loop {
        let input = pwd();
        let command = input.trim();
        let commands = command.split_whitespace().collect::<Vec<&str>>();
        let command_type = BuildInCommand::from(commands[0]);
        match command_type {
            BuildInCommand::Exit => break,
            BuildInCommand::Cd => {
                if commands.len() > 2 {
                    eprintln!("{}: string not in pwd: {}", commands[0], commands[1]);
                    continue;
                }
                if commands.len() == 1 {
                    if cd("/").is_err() {
                        eprintln!("{}: failed to change directory", commands[0]);
                    }
                    continue;
                }
                let path = commands[1];
                if cd(path).is_err() {
                    eprintln!("{}: failed to change directory", commands[0]);
                }
                continue;
            }
            BuildInCommand::Other(cmd) => match Command::new(cmd).args(&commands[1..]).status() {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("{}: command not found", commands[0]);
                }
            },
        }
    }
}

pub fn pwd() -> String {
    let current_dir = Command::new("pwd").output().unwrap().stdout;
    print!("{} > ", String::from_utf8(current_dir).unwrap().trim());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}