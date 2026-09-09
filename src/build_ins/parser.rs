use crate::build_ins::check::BuildInCommand;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub program: BuildInCommand,
    pub arguments: Vec<String>,
    pub input: Option<String>,
    pub output: Option<String>,
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
        } else if character.is_whitespace() || matches!(character, '|' | '<' | '>') {
            if !current.is_empty() {
                tokens.push(Token::Word(current.clone()));
                current.clear();
            }
            match character {
                '|' => tokens.push(Token::Pipe(character)),
                '<' => tokens.push(Token::RedirectInput),
                '>' => tokens.push(Token::RedirectOutput),
                _ => (),
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

pub fn parse_pipeline(tokens: &[Token]) -> Vec<ParsedCommand> {
    let mut commands = Vec::new();
    let mut current_word: Vec<String> = Vec::new();
    let mut input = None;
    let mut output = None;
    let mut index = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Word(word) => current_word.push(word.clone()),
            Token::Pipe(_) => {
                if !current_word.is_empty() {
                    commands.push(build_command(&current_word, input.take(), output.take()));
                    current_word.clear();
                }
            }
            Token::RedirectInput => {
                if let Some(Token::Word(word)) = tokens.get(index + 1) {
                    input = Some(word.clone());
                    index += 1;
                }
            }
            Token::RedirectOutput => {
                if let Some(Token::Word(word)) = tokens.get(index + 1) {
                    output = Some(word.clone());
                    index += 1;
                }
            }
        }
        index += 1;
    }

    if !current_word.is_empty() {
        commands.push(build_command(&current_word, input, output));
    }
    commands
}

fn build_command(words: &[String], input: Option<String>, output: Option<String>) -> ParsedCommand {
    ParsedCommand {
        program: BuildInCommand::from(words[0].as_str()),
        arguments: words[1..].to_vec(),
        input,
        output,
    }
}
