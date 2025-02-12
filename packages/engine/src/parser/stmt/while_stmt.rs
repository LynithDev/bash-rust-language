use crate::{as_stmt_kind, ast, parse, parseable, parser::expr::{Block, Expression}};

ast!(While(Expression, Block));
as_stmt_kind!(While);

parseable! {
    While = |parser| {
        parser.expect_token(&LexerTokenKind::While)?;

        let condition = parser.expression()?;
        parser.next();

        parse!(parser, block = Block);

        Ok(Some(While(condition, block)))
    }
}
