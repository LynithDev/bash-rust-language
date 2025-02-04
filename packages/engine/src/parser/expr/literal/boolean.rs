use crate::{lexer::tokens::LexerToken, parser::ParserErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BooleanLiteral(bool);

impl TryFrom<LexerToken> for BooleanLiteral {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(*value.as_boolean()?))
    }
}