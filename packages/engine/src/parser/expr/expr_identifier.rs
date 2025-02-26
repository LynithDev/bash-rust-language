use crate::{ast, lexer::tokens::LexerToken, parseable, parser::ParserErrorKind, to_expr_kind};

ast!(Identifier(String));
to_expr_kind!(Identifier);

parseable! {
    Identifier = |parser| {
        todo!()
    }
}

impl TryFrom<LexerToken> for Identifier {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(value.as_identifier()?.to_owned()))
    }
}