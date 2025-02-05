use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::{Block, Expression}}};

use super::{variable_stmt::VariableMeta, StatementKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For(VariableMeta, Expression, Block);

impl Parse<StatementKind> for For {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<StatementKind> {
        parser.expect_token(&LexerTokenKind::For)?;

        let identifier = parser
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?;
        let variable = VariableMeta {
            name: identifier.clone(),
            strict_type: None,
            value: None,
        };

        parser.expect_token(&LexerTokenKind::In)?;
        // let_expr!(expr = parser.expression()?);
        let expr = parser.expression()?;
        parser.next();

        let block = parser.stmt_block()?;

        Ok(StatementKind::For((variable, expr, block)))
    }
}