use std::fmt::Display;

#[derive(Debug)]
pub enum BuildInCommand {
    Cd,
    Exit,
    Other(String),
}
impl From<&str> for BuildInCommand {
    fn from(s: &str) -> Self {
        match s {
            "cd" => BuildInCommand::Cd,
            "exit" => BuildInCommand::Exit,
            _ => BuildInCommand::Other(s.to_string()),
        }
    }
}
impl Display for BuildInCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildInCommand::Cd => write!(f, "cd"),
            BuildInCommand::Exit => write!(f, "exit"),
            BuildInCommand::Other(cmd) => write!(f, "{}", cmd),
        }
    }
}