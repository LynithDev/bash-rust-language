use crate::{ast, as_stmt, parseable, parser::expr::Expression};

ast!(Return(Option<Expression>));
as_stmt!(Return);

parseable! {
    Return = |parser| {
        parser.expect_token(&LexerTokenKind::Return)?;

        let value = parser.expression()?;

        Ok(Some(Return(value)))
    }
}