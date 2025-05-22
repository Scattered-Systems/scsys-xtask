/*
    Appellation: error <module>
    Contrib: @FL03
*/

/// a type alias for a [`Result`] type with a default error type of [`Error`]
#[allow(dead_code)]
pub(crate) type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Build Failed: {0}")]
    BuildFailed(String),
    #[error("Invalid output: {0}")]
    InvalidOutput(String),

    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("[Unknown Error] {0}")]
    UnknownError(String),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::UnknownError(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::UnknownError(err.to_string())
    }
}