use crate::{lexer::tokens::LexerTokenKind, parser::ParserErrorKind};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment(pub Expression, pub AssignmentOperator, pub Expression);

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