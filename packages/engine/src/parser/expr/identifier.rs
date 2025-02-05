use crate::{lexer::tokens::LexerToken, parser::ParserErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl TryFrom<LexerToken> for Identifier {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(value.as_identifier()?.to_owned()))
    }
}