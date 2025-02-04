use crate::parser::expr::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStmt(Expression);