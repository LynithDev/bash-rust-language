#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum EngineError {
    #[error("{0}")]
    LexerError(#[from] crate::lexer::LexerError),
    #[error("an unreachable error has occurred. this shouldn't ever happen")]
    Unreachable,
    #[error("an unknown error has occurred")]
    Unknown,
}

pub type EngineResult<T> = std::result::Result<T, EngineError>;
pub type ErrorList = Vec<EngineError>;
