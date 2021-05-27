use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing environment variable: {variable}")]
    MissingEnv {
        #[source]
        source: std::env::VarError,
        variable: String,
    },

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde error: {error}\nMsg: {msg}")]
    Serde {
        error: serde_json::Error,
        msg: String,
    },

    #[error("Invalid request. Received status {0}. Message: {1}")]
    ClientError(reqwest::StatusCode, String),

    #[error("Server error. Received status {0}. Message: {1}")]
    ServerError(reqwest::StatusCode, String),
}

pub type Result<T> = std::result::Result<T, Error>;
