use crate::{as_expr, ast, parseable};

use super::Expression;

ast!(Group(Expression));
as_expr!(Group);

parseable! {
    Group = |parser| {
        let mut expr = parser.expression()?;

        parser.expect_token(&LexerTokenKind::RParen)?;

        expr.value = ExpressionKind::Group(expr.value);

        Ok(expr)
    }
}