use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Unknown error")]
    Unknown,
}

pub type PaymentResult<T> = Result<T, PaymentError>;
