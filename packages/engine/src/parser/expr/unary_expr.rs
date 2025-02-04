use crate::{lexer::tokens::LexerTokenKind, parser::ParserErrorKind};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unary(pub UnaryOperator, pub Expression);

#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Negative,
}

impl TryFrom<LexerTokenKind> for UnaryOperator {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::Not => Self::Not,
            LexerTokenKind::Minus => Self::Negative,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}