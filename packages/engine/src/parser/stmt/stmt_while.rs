use crate::{as_stmt_kind, ast, ok_or_none, parseable, parser::expr::{Block, Expression}};

ast!(While(Expression, Block));
as_stmt_kind!(While);

parseable! {
    While = |parser| {
        parser.expect_token(&LexerTokenKind::While)?;

        let condition = ok_or_none!(parser.expression()?);
        
        parser.next();

        let block = ok_or_none!(Block::parse(parser)?);

        Ok(Some(While(condition, block)))
    }
}
