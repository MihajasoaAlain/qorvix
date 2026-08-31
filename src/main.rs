use std::io::{self, Write};
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

    }
}
