#[derive(thiserror::Error, lang_macro::EnumVariants, Debug)]
pub enum TranspilerErrorKind {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("fmt error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("unexpected error")]
    Unexpected
}

pub(super) type TranspilerResult<T> = std::result::Result<T, TranspilerErrorKind>;