
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
