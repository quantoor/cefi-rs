use std::fmt;

pub type BybitResult<T> = Result<T, BybitError>;

#[derive(Debug)]
pub enum BybitError {
    ApiError(i64, String),
    DeserializeError(String),
    Unknown(String),
}

impl From<anyhow::Error> for BybitError {
    fn from(value: anyhow::Error) -> Self {
        BybitError::Unknown(format!("{}", value))
    }
}

impl From<serde_json::Error> for BybitError {
    fn from(value: serde_json::Error) -> Self {
        BybitError::DeserializeError(format!("{}", value))
    }
}

impl fmt::Display for BybitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
