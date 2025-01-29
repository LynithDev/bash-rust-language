use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone)]
pub enum EngineError {
    #[error("{0}")]
    LexerError(#[from] crate::lexer::LexerError),
    #[error("could not read file: {0}")]
    FileSystemError(#[from] Arc<std::io::Error>),
    #[error("an unreachable error has occurred. this shouldn't ever happen")]
    Unreachable,
    #[error("an unknown error has occurred")]
    UnknownError,
}

pub type EngineResult<T> = std::result::Result<T, EngineError>;
pub type ErrorList = Vec<EngineError>;
