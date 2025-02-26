use crate::parser::{Parser, ParserResult};

use super::Expression;

pub mod or_op;
pub mod and_op;
pub mod cmp_eq_or;

pub trait BinOp<T> {
    fn parse_left(parser: &mut Parser) -> ParserResult<Option<Expression>>;
    fn parse_right(parser: &mut Parser) -> ParserResult<Option<Expression>>;
}

#[macro_export]
macro_rules! parse_bin_op {
    ($name:ident = |$parser:ident| { left = $parse_left:block right = $parse_right:block }) => {
        impl $crate::parser::expr::bin_op::BinOp<$name> for $name {
            fn parse_left($parser: &mut $crate::parser::Parser) -> $crate::parser::ParserResult<Option<Expression>> {
                #[allow(unused_imports)]
                use $crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{expr::ExpressionKind, stmt::StatementKind, ParserErrorKind}};
                
                $parse_left
            }

            fn parse_right($parser: &mut $crate::parser::Parser) -> $crate::parser::ParserResult<Option<Expression>> {
                #[allow(unused_imports)]
                use $crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{expr::ExpressionKind, stmt::StatementKind, ParserErrorKind}};
                
                $parse_right
            }
        }

        $crate::parseable! {
            $name = |parser| {
                use $crate::parser::expr::bin_op::BinOp;
                let lhs = $crate::ok_or_none!($name::parse_left(parser)?);
                let rhs = $crate::ok_or_none!($name::parse_right(parser)?);
                
                Ok(Some($name(lhs, rhs)))
            }
        }
    };
}