use crate::{as_stmt_kind, ast, ok_or_none, parseable, parser::expr::Block};

use super::stmt_variable::VariableMeta;

ast!(Function {
    name: String,
    parameters: Option<Vec<VariableMeta>>,
    strict_type: Option<String>,
    body: Block,
});
as_stmt_kind!(Function);

parseable! {
    Function = |parser| {
        parser.expect_token(&LexerTokenKind::Function)?;

        let identifier = parser
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?
            .clone();

        parser.expect_token(&LexerTokenKind::LParen)?;

        let parameters = if parser.next_if_eq(&&LexerTokenKind::RParen).is_none() {
            let mut variables = vec![];

            while let Some(token) = parser.peek() {
                if token.kind == LexerTokenKind::RParen {
                    break;
                }

                if parser.next_if_eq(&&LexerTokenKind::Comma).is_some() {
                    continue;
                }

                let var = ok_or_none!(VariableMeta::parse(parser)?);
                variables.push(var);
            }

            parser.next();

            Some(variables)
        } else {
            None
        };

        let strict_type = parser.parse_explicit_type()?;

        parser.expect_token(&LexerTokenKind::LBracket)?;

        let body = ok_or_none!(Block::parse(parser)?);

        Ok(Some(Function {
            name: identifier.to_owned(),
            parameters,
            strict_type,
            body,
        }))
    }
}