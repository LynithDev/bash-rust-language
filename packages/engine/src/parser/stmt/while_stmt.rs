use crate::parser::expr::{Block, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While(Expression, Block);