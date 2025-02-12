use crate::{to_expr_kind, ast, lexer::tokens::LexerTokenKind, parseable, parser::{expr::Identifier, ParserErrorKind}};

use super::Expression;

ast!(Assignment(Expression, AssignmentOperator, Expression));
to_expr_kind!(Assignment);

parseable! {
    Assignment = |parser| {
        let lhs = parser.expr_logic_or()?;

        let Some(op_token) = parser.next_if_eq_mul(&[
            &LexerTokenKind::Equal,
            &LexerTokenKind::PlusEqual,
            &LexerTokenKind::MinusEqual,
            &LexerTokenKind::MultiplyEqual,
            &LexerTokenKind::DivideEqual,
        ]) else {
            return Ok(None);
        };

        let rhs = parser.expr_logic_or()?;

        let ExpressionKind::Identifier(Identifier(identifier)) = lhs else {
            return Ok(None);
        };

        let operator: AssignmentOperator = op_token.kind.clone().try_into()?;

        Ok(Some(Assignment(
            lhs,
            operator,
            rhs,
        )))
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