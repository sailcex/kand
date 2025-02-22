#[derive(thiserror::Error, Debug)]
pub enum KandError {
    #[error("Invalid parameter value provided to the function")]
    InvalidParameter,

    #[error("Insufficient data points for the requested calculation")]
    InsufficientData,

    #[error("Input data contains NaN (Not a Number) values")]
    NaNDetected,

    #[error("Input arrays have mismatched lengths")]
    LengthMismatch,

    #[error("Input data is invalid (out of range or empty)")]
    InvalidData,

    #[error("File operation error occurred")]
    FileError,

    #[error("Failed to convert between numeric types")]
    ConversionError,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),
}

pub type Result<T> = std::result::Result<T, KandError>;
