use crate::build_ins::check::BuildInCommand;

pub struct ParsedCommand{
    pub program: BuildInCommand,
    pub arguments: Vec<String>,
}

pub fn parse(input: String)-> Option<ParsedCommand>{
    if input.trim().is_empty() {
        return None;
    }

    let parts : Vec<String> = input.split_whitespace().map(String::from).collect();
    Some(ParsedCommand{
        program: BuildInCommand::from(parts[0].as_str()),
        arguments: parts[1..].to_vec(),
    })
}