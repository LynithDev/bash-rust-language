use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group(pub Expression);