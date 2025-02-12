use crate::{to_expr_kind, ast, lexer::tokens::LexerToken, parser::ParserErrorKind};

ast!(StringLiteral(String));
to_expr_kind!(StringLiteral = String);

impl std::hash::Hash for StringLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl TryFrom<LexerToken> for StringLiteral {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(value.as_string()?.to_owned()))
    }
}