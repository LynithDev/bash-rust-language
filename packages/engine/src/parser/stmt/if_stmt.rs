use crate::parser::expr::IfExpr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStmt(IfExpr);