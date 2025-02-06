use crate::{as_expr, ast, parseable};

use super::{Expression, Identifier};

ast!(FunctionCall(Identifier, Vec<Expression>));
as_expr!(FunctionCall);

parseable! {
    FunctionCall = |parser| {
        let expr = parser.expr_primary()?;

        let ExpressionKind::Identifier(identifier) = &expr.value else {
            return Ok(None);
        };

        if parser.next_if_eq(&&LexerTokenKind::LParen).is_none() {
            return Ok(None);
        }

        let mut args = vec![];
        let mut end = parser.cursor;

        while let Some(token) = parser.peek() {
            dbg!(token);

            if parser.next_if_eq(&&LexerTokenKind::RParen).is_some() {
                end = parser.cursor;
                break;
            }

            if parser.next_if_eq(&&LexerTokenKind::Comma).is_some() {
                continue;
            }

            let arg = parser.expression()?;

            args.push(arg);
        }

        Ok(Some(FunctionCall(identifier, args)))
    }
}