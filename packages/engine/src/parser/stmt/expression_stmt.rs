use crate::{as_stmt_kind, ast, parser::expr::Expression};

ast!(ExpressionStmt(Expression));
as_stmt_kind!(ExpressionStmt = Expression);