use crate::{to_expr_kind, ast, lexer::tokens::LexerToken, parser::ParserErrorKind};

ast!(IntegerLiteral(isize));
to_expr_kind!(IntegerLiteral = Integer);

impl std::hash::Hash for IntegerLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl TryFrom<LexerToken> for IntegerLiteral {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(Self(*value.as_integer()?))
    }
}