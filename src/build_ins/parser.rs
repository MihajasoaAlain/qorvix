use std::fmt::Display;
use crate::build_ins::check::BuildInCommand;

pub struct ParsedCommand{
    pub program: BuildInCommand,
    pub arguments: Vec<String>,
}
#[derive(Debug, Clone)]
pub enum Token
{
    Word(String),
    Pipe(char),
    RedirectOutput,
    RedirectInput
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

pub fn parse(input: String)-> Option<ParsedCommand>{
    if input.trim().is_empty() {
        return None;
    }

    let tokens = lex(input);
    Some(ParsedCommand{
        program: BuildInCommand::from(tokens[0].clone()),
        arguments: tokens[1..].iter().map(|t| t.to_string()).collect(),
    })
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

