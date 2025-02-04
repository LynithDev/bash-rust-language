use std::fmt::Display;

use crate::{cursor::Cursor, error::CodeError, lexer::tokens::LexerTokenKind};

#[derive(thiserror::Error, lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorKind {
    #[error("{0}")]
    EngineError(#[from] crate::error::EngineErrorKind),
    #[error("couldn't convert lexer token {0} to ast node")]
    ConvertError(LexerTokenKind),
    #[error("expected token '{0:?}' but found '{1:?}'")]
    ExpectedToken(Vec<LexerTokenKind>, Option<LexerTokenKind>),
    #[error("expected expression")]
    ExpectedExpression,
    #[error("expected statement")]
    ExpectedStatement,
    #[error("unexpected token '{0:?}'")]
    UnexpectedToken(LexerTokenKind),
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("unknown token")]
    UnknownToken,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "cli"), error("{kind}"))]
pub struct ParserError {
    pub kind: Box<ParserErrorKind>,
    pub start: Cursor,
    pub end: Cursor,

    #[cfg(feature = "cli")]
    pub source_file: crate::error::SourceFile,
}

pub(super) type ParserResult<T> = std::result::Result<T, ParserErrorKind>;

#[cfg(feature = "cli")]
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_error(f)
    }
}

#[cfg(feature = "cli")]
impl CodeError<ParserErrorKind> for ParserError {
    fn kind(&self) -> &ParserErrorKind {
        &self.kind
    }

    fn source_file(&self) -> &crate::error::SourceFile {
        &self.source_file
    }

    fn start(&self) -> &crate::cursor::Cursor {
        &self.start
    }

    fn end(&self) -> &crate::cursor::Cursor {
        &self.end
    }
}