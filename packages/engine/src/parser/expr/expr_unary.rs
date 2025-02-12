use crate::{to_expr_kind, ast, lexer::tokens::LexerTokenKind, parser::ParserErrorKind};

use super::Expression;

ast!(Unary(UnaryOperator, Expression));
to_expr_kind!(Unary);

#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Negative,
}

impl TryFrom<LexerTokenKind> for UnaryOperator {
    type Error = ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::Not => Self::Not,
            LexerTokenKind::Minus => Self::Negative,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}