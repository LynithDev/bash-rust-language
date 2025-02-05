use crate::{lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::Expression}};

use super::StatementKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return(Option<Expression>);

impl Parse<StatementKind> for Return {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<StatementKind> {
        parser.expect_token(&LexerTokenKind::Return)?;

        let value = parser.expression()?;

        Ok(StatementKind::Return(value))
    }
}