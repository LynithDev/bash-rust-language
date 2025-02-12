use crate::{ast, ok_or_none, parseable, to_expr_kind};

use super::Expression;

ast!(Group(Expression));
to_expr_kind!(Group);

parseable! {
    Group = |parser| {
        let expr = ok_or_none!(parser.expression()?);

        parser.expect_token(&LexerTokenKind::RParen)?;

        Ok(Some(Group(expr)))
    }
}