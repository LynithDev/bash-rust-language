use std::fmt::Display;
use crate::error::{CodeError, SourceFile};

use crate::cursor::Cursor;

use super::tokens::LexerTokenKind;

#[derive(thiserror::Error, Debug, lang_macro::EnumVariants, PartialEq, Eq, Clone)]
pub enum LexerErrorKind {
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("invalid number notation (valid notations are b(inary), o(ctal), d(ecimal), x(hex)")]
    InvalidNumberNotation,
    #[error("overflowing integer '{0}'")]
    IntegerOverflow(String),
    #[error("expected character '{expected}' but found {found:?}")]
    ExpectedCharacter { expected: String, found: Option<char> },
    #[error("failed to get value as '{0}' from literal '{1}'")]
    LiteralExtractionError(LexerTokenKind, LexerTokenKind),
    #[error("unknown token")]
    UnknownToken
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub start: Cursor,
    pub end: Cursor,
    pub source_file: SourceFile,
}

pub type LexerResult<T> = std::result::Result<T, LexerErrorKind>;

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_error(f)
    }
}

impl CodeError<LexerErrorKind> for LexerError {
    fn kind(&self) -> &LexerErrorKind {
        &self.kind
    }

    fn start(&self) -> &Cursor {
        &self.start
    }

    fn end(&self) -> &Cursor {
        &self.end
    }

    fn source_file(&self) -> &SourceFile {
        &self.source_file
    }
}