use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{ast::Parse, expr::Literal, ParserErrorKind}};

use super::{Expression, ExpressionKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment(pub Expression, pub AssignmentOperator, pub Expression);

impl Parse<ExpressionKind> for Assignment {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<ExpressionKind> {
        let lhs = parser.expr_logic_or()?;

        if let Some(op_token) = parser.next_if_eq_mul(&[
            &LexerTokenKind::Equal,
            &LexerTokenKind::PlusEqual,
            &LexerTokenKind::MinusEqual,
            &LexerTokenKind::MultiplyEqual,
            &LexerTokenKind::DivideEqual,
        ]) {
            let rhs = parser.expr_logic_or()?;

            if let ExpressionKind::Literal(Literal::Identifier(identifier)) = lhs
            {
                let operator: AssignmentOperator = op_token.kind.clone().try_into()?;

                return Ok(
                    ExpressionKind::Assignment(Assignment(
                        lhs,
                        operator,
                        rhs,
                    )),
                );
            }
        }

        Ok(lhs)
    }
}

#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    Assign
}

impl TryFrom<LexerTokenKind> for AssignmentOperator {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::PlusEqual => Self::PlusAssign,
            LexerTokenKind::MinusEqual => Self::MinusAssign,
            LexerTokenKind::MultiplyEqual => Self::MultiplyAssign,
            LexerTokenKind::DivideEqual => Self::DivideAssign,
            LexerTokenKind::Equal => Self::Assign,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}