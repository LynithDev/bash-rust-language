use crate::{lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::IfExpr}};

use super::StatementKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStmt(IfExpr);

impl Parse<StatementKind> for IfStmt {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<StatementKind> {
        parser.expect_token(&LexerTokenKind::If)?;

        let expr = parser.expr_if()?;

        Ok(StatementKind::If(expr))
    }
}