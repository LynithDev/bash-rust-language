use crate::{as_stmt, ast, parse, parseable, parser::expr::{Block, Expression}};

ast!(While(Expression, Block));
as_stmt!(While);

parseable! {
    While = |parser| {
        parser.expect_token(&LexerTokenKind::While)?;

        let condition = parser.expression()?;
        parser.next();

        parse!(parser, block = Block);

        Ok(Some(While(condition, block)))
    }
}
