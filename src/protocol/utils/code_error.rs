#[derive(Debug, thiserror::Error)]
pub(crate) enum CodeError {
    #[error("Invalid code: {0}")]
    InvalidCode(String),

    #[error("Code conversion error: {0}")]
    CodeConversionError(String),

    #[error("code length error: expected {0}, got {1}")]
    CodeLengthError(usize, usize),

    #[error("usize conversion error: value {0} is out of range for {1}")]
    UsizeConversionError(usize, &'static str),

    #[error("UTF-8 decoding error")]
    UTF8DecodingError,

    #[error("Invalid Code: {0} in MQTT Protocol")]
    MQTTInvalidCode(u32),
}
