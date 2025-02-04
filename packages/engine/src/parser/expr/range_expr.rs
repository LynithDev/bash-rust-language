use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range(pub Expression, pub Expression, pub bool);