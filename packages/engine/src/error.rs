use colored::Colorize;

use crate::cursor::Cursor;

#[derive(thiserror::Error, Debug)]
pub enum EngineErrorKind {
    #[error("{0}")]
    LexerError(#[from] crate::lexer::LexerError),
    #[error("{0}")]
    ParserError(#[from] crate::parser::ParserError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("exec error")]
    ExecError,
    #[error("expected file")]
    ExpectedFileError,
    #[error("an unreachable error has occurred. this shouldn't ever happen")]
    Unreachable,
    #[error("an unknown error has occurred")]
    UnknownError,
}

pub type EngineResult<T> = std::result::Result<T, EngineErrorKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    path: String,
    code: String,
}

impl SourceFile {
    pub fn from(code: String, path: Option<String>) -> Self {
        Self {
            code,
            path: path.unwrap_or(String::from("virtual"))
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn sliced(&self, start: Cursor, end: Cursor) -> SourceFile {
        let start_index = start.index() as usize;
        let end_index = end.index() as usize;

        let code = self.get_code();
        let slice = &code[start_index..end_index.min(code.len())];

        SourceFile::from(slice.to_string(), Some(self.path.clone()))
    }
}

pub trait CodeError<T>
where T: lang_macro::EnumVariantsTrait + ToString {
    fn kind(&self) -> &T;
    fn source_file(&self) -> &SourceFile;
    fn start(&self) -> &crate::cursor::Cursor;
    fn end(&self) -> &crate::cursor::Cursor;

    fn format_error(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, 
            "{}: [{}] {}",
            " error ".on_red(),
            self.kind().variant_name().bright_red().bold(),
            self.kind().to_string().bold(),
        )?;

        let path = self.source_file().get_path();
        let code = self.source_file().get_code();

        writeln!(f,
            "  {} {}",
            "at".black(),
            format!(
                "{}{}{}{}{}",
                path,
                ":".black(),
                self.start().line(),
                ":".black(),
                self.start().col(),
            ).bold()
        )?;

        let lines = (self.end().line() - self.start().line()) + 1;
        let max_line_len = self.end().line().to_string().len();

        writeln!(f,
            "   {} {}",
            " ".repeat(max_line_len),
            "|".bright_blue().bold(),
        )?;

        for index in 0..lines {
            let line = (self.start().line() + index).to_string();
            let line_len = line.len();

            writeln!(f,
                "   {}{} {}  {}",
                " ".repeat(max_line_len - line_len),
                line,
                "|".bright_blue().bold(),
                code.trim()
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
