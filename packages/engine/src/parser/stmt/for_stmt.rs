use crate::parser::expr::{Block, Expression};

use super::variable_stmt::VariableMeta;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For(VariableMeta, Expression, Block);