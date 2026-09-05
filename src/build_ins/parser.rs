use crate::build_ins::check::BuildInCommand;
use std::fmt::Display;

pub struct ParsedCommand {
    pub program: BuildInCommand,
    pub arguments: Vec<String>,
}
#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Pipe(char),
    RedirectOutput,
    RedirectInput,
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::Word(s) => s.clone(),
            Token::Pipe(c) => c.to_string(),
            Token::RedirectOutput => ">".to_string(),
            Token::RedirectInput => "<".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub fn parse(input: String) -> Option<ParsedCommand> {
    if input.trim().is_empty() {
        return None;
    }

    let tokens = lex(input);
   match parsing(&tokens) {
        Some(command) => Option::from(ParsedCommand {
            program: command.program,
            arguments: command.arguments,
        }),
        _ => return None,
    }
}
pub fn lex(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for character in input.chars() {
        if character == '"' {
            in_quotes = !in_quotes;
        } else if in_quotes {
            current.push(character);
        } else if character.is_whitespace() {
            if !current.is_empty() {
                tokens.push(Token::Word(current.clone()));
                current.clear();
            }
        } else {
            current.push(character);
        }
    }
    if !current.is_empty() {
        tokens.push(Token::Word(current));
    }
    tokens
}

pub fn parsing(tokens: &[Token]) -> Option<ParsedCommand> {
let mut words = Vec::new();
for token in tokens {
    match token {
        Token::Word(word) => words.push(word.clone()),
        _ => (),
    }
}
if words.is_empty() {
    None
} else {
    Some(ParsedCommand {
        program: BuildInCommand::from(words[0].as_str()),
        arguments: words[1..].to_vec(),
    })
}

}