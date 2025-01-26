#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("lexer error: {0}")]
    LexerError(#[from] crate::lexer::LexerError),
    #[error("an unknown error has occurred")]
    Unknown
}

pub type EngineResult<T> = std::result::Result<T, EngineError>;