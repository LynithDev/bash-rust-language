use crate::{lexer::tokens::LexerTokenKind, parser::ParserErrorKind};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logic(pub Expression, pub LogicalOperator, pub Expression);

#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    Equal,
    NotEqual,
    LesserThan,
    LesserEqualThan,
    GreaterThan,
    GreaterEqualThan,
    And,
    Or
}

impl TryFrom<LexerTokenKind> for LogicalOperator {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::EqualEqual => Self::Equal,
            LexerTokenKind::NotEqual => Self::NotEqual,
            LexerTokenKind::LesserThan => Self::LesserThan,
            LexerTokenKind::LesserEqualThan => Self::LesserEqualThan,
            LexerTokenKind::GreaterThan => Self::GreaterThan,
            LexerTokenKind::GreaterEqualThan => Self::GreaterEqualThan,
            LexerTokenKind::And => Self::And,
            LexerTokenKind::Or => Self::Or,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}