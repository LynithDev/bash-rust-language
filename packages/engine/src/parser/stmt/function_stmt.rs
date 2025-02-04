use crate::parser::expr::Block;

use super::variable_stmt::VariableMeta;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Option<Vec<VariableMeta>>,
    pub strict_type: Option<String>,
    pub body: Block,
}