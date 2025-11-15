#[derive(Debug, thiserror::Error)]
pub(crate) enum CodeError {
    #[error("Invalid code: {0}")]
    InvalidCode(String),

    #[error("Code conversion error: {0}")]
    CodeConversionError(String),

    #[error("code length error: expected {0}, got {1}")]
    CodeLengthError(usize, usize),
}