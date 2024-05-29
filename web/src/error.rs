// external crate imports
use thiserror::Error;

// standard library imports
use std::sync::{self, mpsc};

// ----- defining custom error enum for `ThreadPool`
#[derive(Debug, Error)]
pub enum ThreadPoolError {
    #[error("Receiver lock error: {0}")]
    ReceiverLockError(String),

    #[error("Receive error: {0}")]
    ReceiveError(#[from] mpsc::RecvError),

    #[error("Send error: {0}")]
    SendError(String),
}
// implement conversion from `PosionError` to `ThreadPoolError::ReceiverLockError`
impl<T> From<sync::PoisonError<T>> for ThreadPoolError {
    fn from(err: sync::PoisonError<T>) -> Self {
        ThreadPoolError::ReceiverLockError(err.to_string())
    }
}

// ----- defining custom error enum for `Request`
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Invalid request line: {0}")]
    InvalidRequestLineError(String),

    #[error("Empty HTTP request")]
    EmptyRequestError,
}

// ----- defining custom error enum for `WebServer`
#[derive(Debug, Error)]
pub enum WebServerError {
    #[error("Stream flush error: {0}")]
    StreamFlushError(String),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Request parse error: {0}")]
    RequestParseError(RequestError),
}
