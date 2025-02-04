use super::{Block, else_expr::ElseExpr, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExpr(pub Expression, pub Block, pub Option<ElseExpr>);