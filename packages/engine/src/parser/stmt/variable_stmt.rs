use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::Expression, Parser, ParserResult}};

use super::StatementKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable(pub VariableMeta);

impl Parse<StatementKind> for Variable {
    fn parse(parser: &mut Parser) -> ParserResult<StatementKind> {
        parser.expect_token(&LexerTokenKind::Var)?;
        let value = VariableMeta::parse(parser)?;
        parser.expect_terminator()?;

        Ok(StatementKind::Variable(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableMeta {
    pub name: String,
    pub strict_type: Option<String>,
    pub value: Option<Expression>,
}

impl Parse<VariableMeta> for VariableMeta {
    fn parse(parser: &mut Parser) -> ParserResult<VariableMeta> {
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

        Ok(VariableMeta {
            name: identifier.to_owned(),
            strict_type,
            value,
        })
    }
}