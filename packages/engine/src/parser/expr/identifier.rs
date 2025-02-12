use crate::{to_expr_kind, ast, lexer::tokens::LexerToken, parser::ParserErrorKind};

ast!(Identifier(String));
to_expr_kind!(Identifier);

impl TryFrom<LexerToken> for Identifier {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(value.as_identifier()?.to_owned()))
    }
}