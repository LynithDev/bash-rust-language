use crate::{lexer::tokens::LexerToken, parser::ParserErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntegerLiteral(isize);

impl TryFrom<LexerToken> for IntegerLiteral {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(*value.as_integer()?))
    }
}