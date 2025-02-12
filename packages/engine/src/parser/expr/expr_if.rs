use crate::{ast, ok_or_none, parseable, to_expr_kind};

use super::{Block, Expression};

ast!(IfExpr(Expression, Block, Option<Expression>));
to_expr_kind!(IfExpr = If);

parseable! {
    IfExpr = |parser| {
        let condition = ok_or_none!(parser.expression()?);

        let truthy_block = if parser.next_if_eq(&&LexerTokenKind::LBracket).is_some() {
            ok_or_none!(Block::parse(parser)?)
        } 
        // TODO:
        // else if parser.next_if_eq(&&LexerTokenKind::Colon).is_some() {
        //     parser.parse_inline_block()?
        // } 
        else {
            return Err(ParserErrorKind::ExpectedExpression);
        };

        // let else_condition = if parser.next_if_eq(&&LexerTokenKind::Else).is_some() {
        //     let start = parser.cursor;

        //     Some(match parser.peek().map(|t| t.kind.clone()) {
        //         Some(LexerTokenKind::LBracket) => {
        //             parser.next();
        //             Block::parse(parser)?
        //         }
        //         // Some(LexerTokenKind::Colon) => {
        //         //     parser.next();
        //         //     parser.parse_inline_block()?
        //         // }
        //         _ => {
        //             todo!("if else parsing");
        //             // let stmt = IfStmt::parse(parser)?;
        //             // vec![stmt]
        //         }
        //     })
        // } else {
        //     None
        // };

        Ok(Some(IfExpr(condition, truthy_block, None)))
    }
}