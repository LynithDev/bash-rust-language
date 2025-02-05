use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::{Block, Expression}}};

use super::StatementKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While(Expression, Block);

impl Parse<StatementKind> for While {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<StatementKind> {
        parser.expect_token(&LexerTokenKind::While)?;

        let condition = parser.expression()?;
        parser.next();

        let block = parser.stmt_block()?;

        Ok(StatementKind::While((condition, block)))
    
    }
}