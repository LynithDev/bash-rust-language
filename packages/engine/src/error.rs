#[cfg(feature = "cli")]
use colored::Colorize;

use crate::lexer::tokens::LexerTokenKind;

#[derive(thiserror::Error, PartialEq, Eq, Debug, Clone)]
pub enum EngineErrorKind {
    #[error("{0}")]
    LexerError(#[from] crate::lexer::LexerError),
    #[error("{0}")]
    ParserError(#[from] crate::parser::ParserError),
    #[error("failed to get value as '{0}' from literal '{1}'")]
    LiteralExtractionError(LexerTokenKind, LexerTokenKind),
    #[error("an unreachable error has occurred. this shouldn't ever happen")]
    Unreachable,
    #[error("an unknown error has occurred")]
    UnknownError,
}

pub type EngineResult<T> = std::result::Result<T, EngineErrorKind>;
pub type ErrorList = Vec<EngineErrorKind>;

#[cfg(feature = "cli")]
pub(super) type SourceFile = Box<(Option<std::path::PathBuf>, String)>;

#[cfg(feature = "cli")]
pub trait CodeError<T>
where T: lang_macro::EnumVariantsTrait + ToString {
    fn kind(&self) -> &T;
    fn source_file(&self) -> &SourceFile;
    fn start(&self) -> &crate::Cursor;
    fn end(&self) -> &crate::Cursor;

    fn format_error(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, 
            "{}: [{}] {}",
            " error ".on_red(),
            self.kind().variant_name().bright_red().bold(),
            self.kind().to_string().bold(),
        )?;

        let (path, source) = *self.source_file().clone();
        let (path, source) = (path.map_or("VM".to_string(), |path| path.to_string_lossy().to_string()), source);

        writeln!(f,
            "  {} {}",
            "at".black(),
            format!(
                "{}{}{}{}{}",
                path,
                ":".black(),
                self.start().line,
                ":".black(),
                self.start().col,
            ).bold()
        )?;

        let lines = (self.end().line - self.start().line) + 1;
        let max_line_len = self.end().line.to_string().len();

        writeln!(f,
            "   {} {}",
            " ".repeat(max_line_len),
            "|".bright_blue().bold(),
        )?;

        for index in 0..lines {
            let line = (self.start().line + index).to_string();
            let line_len = line.len();

            writeln!(f,
                "   {}{} {}  {}",
                " ".repeat(max_line_len - line_len),
                line,
                "|".bright_blue().bold(),
                source.trim()
            )?;
        }

        let width = self.end().index() - self.start().index();

        writeln!(f,
            "   {} {}  {}",
            " ".repeat(max_line_len),
            "|".bright_blue().bold(),
            "^".repeat(width as usize).bright_red()
        )?;


        Ok(())
    }
}
