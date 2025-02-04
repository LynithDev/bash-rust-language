use super::{literal::identifier::IdentifierLiteral, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall(pub IdentifierLiteral, pub Vec<Expression>);