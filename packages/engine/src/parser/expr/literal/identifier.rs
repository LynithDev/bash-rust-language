use crate::{lexer::tokens::LexerToken, parser::ParserErrorKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierLiteral(String);

impl TryFrom<LexerToken> for IdentifierLiteral {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(value.as_identifier()?.to_owned()))
    }
}