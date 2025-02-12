use crate::{to_expr_kind, ast};

use super::Expression;

ast!(Range(Expression, Expression, bool));
to_expr_kind!(Range);