use crate::{as_stmt_kind, ast};

use super::variable_stmt::VariableMeta;

ast!(Constant(VariableMeta));
as_stmt_kind!(Constant);