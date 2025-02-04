use crate::{lexer::tokens::LexerTokenKind, parser::ParserErrorKind};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arithmetic(pub Expression, pub ArithmeticOperator, pub Expression);

#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl TryFrom<LexerTokenKind> for ArithmeticOperator {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::Plus => Self::Add,
            LexerTokenKind::Minus => Self::Subtract,
            LexerTokenKind::Multiply => Self::Multiply,
            LexerTokenKind::Divide => Self::Divide,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}