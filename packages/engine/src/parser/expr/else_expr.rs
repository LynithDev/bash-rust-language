use super::{Block, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseExpr(pub Option<Expression>, pub Block, pub Option<Box<ElseExpr>>);