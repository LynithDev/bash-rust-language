use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::Block}};

use super::{variable_stmt::VariableMeta, StatementKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Option<Vec<VariableMeta>>,
    pub strict_type: Option<String>,
    pub body: Block,
}

impl Parse<StatementKind> for Function {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<StatementKind> {
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

                variables.push(VariableMeta::parse(parser)?);
            }

            parser.next();

            Some(variables)
        } else {
            None
        };

        let strict_type = parser.parse_explicit_type()?;

        parser.expect_token(&LexerTokenKind::LBracket)?;

        let body = parser.stmt_block()?;

        let function = Function {
            name: identifier.to_owned(),
            parameters,
            strict_type,
            body,
        };

        Ok(StatementKind::Function(function))
    }
}