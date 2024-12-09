use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid input")]
    InvalidInput,

    #[error("Network error: `{0}`")]
    NetworkError(String),
    #[error("Non exists id")]
    NonExistsId,
    #[error("Unknown error; `{0}`")]
    Unknown(String),
}
