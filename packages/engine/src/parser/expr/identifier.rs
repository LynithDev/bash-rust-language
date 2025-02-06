use crate::{as_expr, ast, lexer::tokens::LexerToken, parser::ParserErrorKind};

ast!(Identifier(String));
as_expr!(Identifier);

impl TryFrom<LexerToken> for Identifier {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(value.as_identifier()?.to_owned()))
    }
}