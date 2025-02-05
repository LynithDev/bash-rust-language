use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{ast::Parse, stmt::Statement}};

use super::ExpressionKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block(pub Vec<Statement>);

impl Parse<ExpressionKind> for Block {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<ExpressionKind> {
        parser.expect_terminator()?;

        let mut statements = vec![];

        while let Some(token) = parser.peek().cloned() {
            if token.kind == LexerTokenKind::RBracket {
                parser.next();
                break;
            }

            let statement = parser.parse_statement(token)?;
            statements.push(statement);

            parser.next();
        }

        Ok(ExpressionKind::Block(Block(statements)))
    }
}