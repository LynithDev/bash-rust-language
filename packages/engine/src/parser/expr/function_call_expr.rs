use super::{Expression, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall(pub Identifier, pub Vec<Expression>);