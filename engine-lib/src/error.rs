#[derive(thiserror::Error, Debug, Clone)]
pub enum EngineError {
    #[error("lexer error: {0}")]
    LexerError(#[from] crate::lexer::LexerError),
    #[error("an unreachable error has occurred. this shouldn't ever happen")]
    Unreachable,
    #[error("int parse error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("an unknown error has occurred")]
    Unknown,
}

pub type EngineResult<T> = std::result::Result<T, EngineError>;
pub type ErrorList = Vec<EngineError>;