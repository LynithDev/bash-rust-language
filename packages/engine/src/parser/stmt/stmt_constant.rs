use crate::{as_stmt_kind, ast};

use super::VariableMeta;

ast!(Constant(VariableMeta));
as_stmt_kind!(Constant);