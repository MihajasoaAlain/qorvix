use crate::build_ins::parser;
use std::process::{Command, Stdio};

pub fn execute_pipeline(commands: &[parser::ParsedCommand]) {
    if commands.is_empty() {
        return;
    }
    let mut previous_stdout = None;
    for (index, command) in commands.iter().enumerate() {
        let mut process = Command::new(command.program.to_string());
        process.args(&command.arguments);
        if let Some(output) = previous_stdout {
            process.stdin(Stdio::from(output));
        }
        if index < commands.len() - 1 {
            process.stdout(Stdio::piped());
        }

        let mut child = match process.spawn() {
            Ok(child) => child,
            Err(_error) => {
                eprintln!("qorvix: {}: command not found", command.program);
                return;
            }
        };
        previous_stdout = child.stdout.take();
        if index == commands.len() - 1 {
            let _ = child.wait();
        }
    }
}
