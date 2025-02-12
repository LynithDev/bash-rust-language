use crate::{as_stmt_kind, ast, parse, parseable, parser::expr::{Block, Expression}};

use super::variable_stmt::VariableMeta;

ast!(For(VariableMeta, Expression, Block));
as_stmt_kind!(For);

parseable! {
    For = |parser| {
        parser.expect_token(&LexerTokenKind::For)?;

        let identifier = parser
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?;
        let variable = VariableMeta {
            name: identifier.clone(),
            strict_type: None,
            value: None,
        };

        parser.expect_token(&LexerTokenKind::In)?;
        // let_expr!(expr = parser.expression()?);
        let expr = parser.expression()?;
        parser.next();

        parse!(parser, block = Block);

        Ok(Some(For(variable, expr, block)))
    }
}
