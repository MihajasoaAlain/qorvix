use crate::build_ins::check::BuildInCommand;
pub(crate) use crate::build_ins::pipeline::execute_pipeline;
use std::fmt::Display;

pub struct Pipeline {
    pub commands: Vec<ParsedCommand>,
}
#[derive(Debug, Clone)]
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

pub fn parse(input: String) -> Option<Vec<ParsedCommand>> {
    if input.trim().is_empty() {
        return None;
    }
    Some(parse_pipeline(&lex(input)))
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
        } else if character == '|' {
            tokens.push(Token::Pipe(character));
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
        if let Token::Word(word) = token { words.push(word.clone()) }
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

pub fn parse_pipeline(tokens: &[Token]) -> Vec<ParsedCommand> {
    let mut commands = Vec::new();
    let mut current_word = Vec::new();
    for token in tokens {
        match token {
            Token::Word(word) => current_word.push(word.clone()),
            Token::Pipe(_word)
                if !current_word.is_empty() => {
                    commands.push(ParsedCommand {
                        program: BuildInCommand::from(current_word[0].as_str()),
                        arguments: current_word[1..].to_vec(),
                    });
                    current_word.clear();
                }
            _ => (),
        }
    }

    if !current_word.is_empty() {
        commands.push(ParsedCommand {
            program: BuildInCommand::from(current_word[0].as_str()),
            arguments: current_word[1..].to_vec(),
        })
    }
    commands
}
