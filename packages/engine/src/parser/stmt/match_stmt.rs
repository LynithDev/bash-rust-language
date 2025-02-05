use crate::{lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::MatchExpr}};

use super::StatementKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchStmt(MatchExpr);

impl Parse<StatementKind> for MatchStmt {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<StatementKind> {
        parser.expect_token(&LexerTokenKind::Match)?;

        let expr = parser.expr_match()?;

        Ok(StatementKind::Match(expr))
    }
}