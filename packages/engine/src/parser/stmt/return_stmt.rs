use crate::{ast, as_stmt_kind, parseable, parser::expr::Expression};

ast!(Return(Option<Expression>));
as_stmt_kind!(Return);

parseable! {
    Return = |parser| {
        parser.expect_token(&LexerTokenKind::Return)?;

        let value = parser.expression()?;

        Ok(Some(Return(value)))
    }
}