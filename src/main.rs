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
        let commands = command.split_whitespace().collect::<Vec<&str>>();
        match Command::new(commands[0]).args(&commands[1..]).status(){
            Ok(_) => (),
            Err(_) => println!("qorvix: Command not found"),
        }
    }
}
