use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Qorvix Shell");

    println!("Type 'exit' to quit the shell.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command == "exit" {
            break;
        }
        match Command::new(command).status(){
            Ok(_) => (),
            Err(_) => println!("qorvix: Command not found"),
        }
    }
}
