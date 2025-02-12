use crate::{to_expr_kind, ast, lexer::tokens::LexerToken, parser::ParserErrorKind};

ast!(BooleanLiteral(bool));
to_expr_kind!(BooleanLiteral = Boolean);

impl std::hash::Hash for BooleanLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl TryFrom<LexerToken> for BooleanLiteral {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(*value.as_boolean()?))
    }
}