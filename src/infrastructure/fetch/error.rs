use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FetchError {
    #[error("failed to send http request:`{0}`")]
    SendRequest(String),
    #[error("client error:`{0}`")]
    Client(String),
    #[error("server error:`{0}`")]
    Server(String),
    #[error("failed to deserialize:`{0}`")]
    Deserialize(String),
    #[error("failed to parse:`{0}`")]
    Parse(String),
}

impl From<FetchError> for crate::Error {
    fn from(value: FetchError) -> Self {
        crate::Error::NetworkError(value.to_string())
    }
}
