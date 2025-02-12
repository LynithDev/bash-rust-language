use crate::{as_stmt_kind, ast, parse, parseable, parser::expr::Block};

use super::variable_stmt::VariableMeta;

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

                parse!(parser, var = VariableMeta);
                variables.push(var);
            }

            parser.next();

            Some(variables)
        } else {
            None
        };

        let strict_type = parser.parse_explicit_type()?;

        parser.expect_token(&LexerTokenKind::LBracket)?;

        parse!(parser, body = Block);

        Ok(Some(Function {
            name: identifier.to_owned(),
            parameters,
            strict_type,
            body,
        }))
    }
}