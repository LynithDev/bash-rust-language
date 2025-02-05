use boolean::BooleanLiteral;
use integer::IntegerLiteral;
use string::StringLiteral;

use crate::{lexer::tokens::{LexerToken, LexerTokenKind}, parser::ParserErrorKind};

pub mod boolean;
pub mod string;
pub mod integer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    String(StringLiteral),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
}

impl TryFrom<LexerToken> for Literal {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(match value.kind {
            LexerTokenKind::String => Literal::String(StringLiteral::try_from(value)?),
            LexerTokenKind::Integer => Literal::Integer(IntegerLiteral::try_from(value)?),
            LexerTokenKind::Boolean => Literal::Boolean(BooleanLiteral::try_from(value)?),
            _ => return Err(ParserErrorKind::ConvertError(value.kind))
        })
    }
}