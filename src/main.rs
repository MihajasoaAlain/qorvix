mod build_ins;
use build_ins::*;

fn main() {
    const GREETINGS: &str = "Hello, and welcome to the Rust Shell!";
    const INFO: &str = "Type 'help' to see the list of commands";
    intro(GREETINGS, INFO);
    execute_command();
}