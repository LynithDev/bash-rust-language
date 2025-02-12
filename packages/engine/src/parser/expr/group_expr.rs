use crate::{to_expr_kind, ast, parseable};

use super::Expression;

ast!(Group(Expression));
to_expr_kind!(Group);

parseable! {
    Group = |parser| {
        let mut expr = parser.expression()?;

        parser.expect_token(&LexerTokenKind::RParen)?;

        expr.value = ExpressionKind::Group(expr.value);

        Ok(expr)
    }
}