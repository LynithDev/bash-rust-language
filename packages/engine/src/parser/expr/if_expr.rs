use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{ast::Parse, ParserErrorKind}};

use super::{else_expr::ElseExpr, Block, Expression, ExpressionKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExpr(pub Expression, pub Block, pub Option<ElseExpr>);

impl Parse<ExpressionKind> for IfExpr {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<ExpressionKind> {
        let condition = parser.expression()?;

        let start = parser.cursor;

        let truthy_block = if parser.next_if_eq(&&LexerTokenKind::LBracket).is_some() {
            parser.stmt_block()?
        } 
        // TODO:
        // else if parser.next_if_eq(&&LexerTokenKind::Colon).is_some() {
        //     parser.parse_inline_block()?
        // } 
        else {
            return Err(ParserErrorKind::ExpectedExpression);
        };

        let else_condition = if parser.next_if_eq(&&LexerTokenKind::Else).is_some() {
            let start = parser.cursor;

            Some(match parser.peek().map(|t| t.kind.clone()) {
                Some(LexerTokenKind::LBracket) => {
                    parser.next();
                    parser.stmt_block()?
                }
                // Some(LexerTokenKind::Colon) => {
                //     parser.next();
                //     parser.parse_inline_block()?
                // }
                _ => {
                    let stmt = parser.stmt_if()?;
                    vec![stmt]
                }
            })
        } else {
            None
        };

        Ok(ExpressionKind::If((condition, truthy_block, else_condition)))
    }
}