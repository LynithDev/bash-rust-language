use std::fmt::Display;

use crate::{cursor::Cursor, error::{CodeError, SourceFile}, lexer::tokens::LexerTokenKind};

#[derive(thiserror::Error, lang_macro::EnumVariants, Debug, PartialEq, Eq, Clone)]
pub enum ParserErrorKind {
    #[error("{0}")]
    EngineError(#[from] crate::lexer::LexerErrorKind),
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

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub struct ParserError {
    pub kind: Box<ParserErrorKind>,
    pub start: Cursor,
    pub end: Cursor,
    pub source_file: SourceFile,
}

pub type ParserResult<T> = std::result::Result<T, ParserErrorKind>;

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_error(f)
    }
}

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