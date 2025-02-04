use crate::parser::expr::MatchExpr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchStmt(MatchExpr);