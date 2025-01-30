#[cfg(feature = "cli")] 
use std::fmt::Display;

#[cfg(feature = "cli")]
use crate::error::{CodeError, SourceFile};

use crate::cursor::Cursor;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, lang_macro::EnumVariants)]
pub enum LexerErrorKind {
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("invalid number notation (valid notations are b(inary), o(ctal), d(ecimal), x(hex)")]
    InvalidNumberNotation,
    #[error("overflowing integer '{0}'")]
    IntegerOverflow(String),
    #[error("expected character '{expected}' but found {found:?}")]
    UnexpectedCharacter { expected: String, found: Option<char> },
    #[error("unknown token")]
    UnknownToken
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "cli"), error("{kind}"))]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub start: Cursor,
    pub end: Cursor,

    #[cfg(feature = "cli")]
    pub source_file: SourceFile,
}

pub(super) type LexerResult<T> = std::result::Result<T, LexerErrorKind>;

#[cfg(feature = "cli")]
impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_error(f)
    }
}

#[cfg(feature = "cli")]
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