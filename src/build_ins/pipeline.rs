use crate::build_ins::parser;
use std::fs::File;
use std::process::{Command, Stdio};

pub fn execute_pipeline(commands: &[parser::ParsedCommand]) {
    if commands.is_empty() {
        return;
    }
    let mut children = Vec::new();
    let mut previous_stdout = None;

    for (index, command) in commands.iter().enumerate() {
        let mut process = Command::new(command.program.to_string());
        process.args(&command.arguments);

        match &command.input {
            Some(path) => match File::open(path) {
                Ok(file) => {
                    process.stdin(Stdio::from(file));
                }
                Err(error) => {
                    eprintln!("qorvix: {}: {}", path, error);
                    break;
                }
            },
            None => {
                if let Some(stdout) = previous_stdout.take() {
                    process.stdin(Stdio::from(stdout));
                }
            }
        }

        match &command.output {
            Some(path) => match File::create(path) {
                Ok(file) => {
                    process.stdout(Stdio::from(file));
                }
                Err(error) => {
                    eprintln!("qorvix: {}: {}", path, error);
                    break;
                }
            },
            None => {
                if index < commands.len() - 1 {
                    process.stdout(Stdio::piped());
                }
            }
        }

        let mut child = match process.spawn() {
            Ok(child) => child,
            Err(_error) => {
                eprintln!("qorvix: {}: command not found", command.program);
                break;
            }
        };
        previous_stdout = child.stdout.take();
        children.push(child);
    }

    for mut child in children {
        let _ = child.wait();
    }
}
