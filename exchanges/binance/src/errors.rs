use std::fmt;

pub type BinanceResult<T> = Result<T, BinanceError>;

#[derive(Debug)]
pub enum BinanceError {
    ApiError(i64, String),
    DeserializeError(String),
    Unknown(String),
}

impl From<anyhow::Error> for BinanceError {
    fn from(value: anyhow::Error) -> Self {
        BinanceError::Unknown(format!("{}", value))
    }
}

impl From<serde_json::Error> for BinanceError {
    fn from(value: serde_json::Error) -> Self {
        BinanceError::DeserializeError(format!("{}", value))
    }
}

impl fmt::Display for BinanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
