#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellCommand(pub String, pub Option<String>);