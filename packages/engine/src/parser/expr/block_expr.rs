use crate::parser::stmt::Statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block(pub Vec<Statement>);