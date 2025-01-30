#[cfg(feature = "cli")]
use colored::Colorize;

#[cfg(feature = "cli")]
use lang_macro::EnumVariantsTrait;

use crate::{cursor::Cursor, lexer::tokens::LexerTokenKind};

#[derive(thiserror::Error, lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorKind {
    #[error("{0}")]
    EngineError(#[from] crate::error::EngineErrorKind),
    #[error("couldn't convert lexer token {0} to ast node")]
    ConvertError(LexerTokenKind),
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("expected token '{0:?}' but found {1:?}")]
    UnexpectedToken(LexerTokenKind, Option<LexerTokenKind>),
    #[error("unknown token")]
    UnknownToken
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "cli"), error("{kind}"))]
pub struct ParserError {
    pub kind: Box<ParserErrorKind>,
    pub start: Cursor,
    pub end: Cursor,

    #[cfg(feature = "cli")]
    pub source_file: SourceFile,
}

#[cfg(feature = "cli")]
pub(super) type SourceFile = Box<(Option<std::path::PathBuf>, String)>;
pub(super) type ParserResult<T> = std::result::Result<T, ParserErrorKind>;


#[cfg(feature = "cli")]
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, 
            "{}: [{}] {}",
            " error ".on_red(),
            self.kind.variant_name().bright_red().bold(),
            self.kind.to_string().bold(),
        )?;

        let (path, source) = *self.source_file.clone();
        let (path, source) = (path.map_or("VM".to_string(), |path| path.to_string_lossy().to_string()), source);

        writeln!(f,
            "  {} {}",
            "at".black(),
            format!(
                "{}{}{}{}{}",
                path,
                ":".black(),
                self.start.line,
                ":".black(),
                self.start.col,
            ).bold()
        )?;

        let lines = (self.end.line - self.start.line) + 1;
        let max_line_len = self.end.line.to_string().len();

        writeln!(f,
            "   {} {}",
            " ".repeat(max_line_len),
            "|".bright_blue().bold(),
        )?;

        for index in 0..lines {
            let line = (self.start.line + index).to_string();
            let line_len = line.len();

            writeln!(f,
                "   {}{} {}  {}",
                " ".repeat(max_line_len - line_len),
                line,
                "|".bright_blue().bold(),
                source.trim()
            )?;
        }

        let width = self.end.index() - self.start.index();

        writeln!(f,
            "   {} {}  {}",
            " ".repeat(max_line_len),
            "|".bright_blue().bold(),
            "^".repeat(width as usize).bright_red()
        )?;


        Ok(())
    }
}