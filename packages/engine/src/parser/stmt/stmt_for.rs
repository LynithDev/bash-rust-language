use crate::{as_stmt_kind, ast, ok_or_none, parseable, parser::expr::{Block, Expression}};

use super::VariableMeta;

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

        let expr = ok_or_none!(parser.expression()?);

        parser.next();

        let block = ok_or_none!(Block::parse(parser)?);

        Ok(Some(For(variable, expr, block)))
    }
}
