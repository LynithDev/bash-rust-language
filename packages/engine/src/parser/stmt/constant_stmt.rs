use super::variable_stmt::VariableMeta;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant(VariableMeta);