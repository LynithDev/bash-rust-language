use crate::{as_stmt_kind, ast, ok_or_none, parseable, parser::expr::Expression};

ast!(Variable(VariableMeta));
as_stmt_kind!(Variable);

parseable! {
    Variable = |parser| {
        parser.expect_token(&LexerTokenKind::Var)?;
        let value = ok_or_none!(VariableMeta::parse(parser)?);
        parser.expect_terminator()?;

        Ok(Some(Variable(value)))
    }
}

ast!(VariableMeta {
    name: String,
    strict_type: Option<String>,
    value: Option<Expression>
});

parseable! {
    VariableMeta = |parser| {
        let identifier = parser
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?
            .clone();

        let strict_type = parser.parse_explicit_type()?;

        let value = if parser.next_if_eq(&&LexerTokenKind::Equal).is_some() {
            parser.expression()?
        } else {
            None
        };

        Ok(Some(VariableMeta {
            name: identifier.to_owned(),
            strict_type,
            value,
        }))
    }
}