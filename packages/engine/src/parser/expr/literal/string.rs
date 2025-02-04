use crate::{lexer::tokens::LexerToken, parser::ParserErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringLiteral(String);

impl TryFrom<LexerToken> for StringLiteral {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(value.as_string()?.to_owned()))
    }
}