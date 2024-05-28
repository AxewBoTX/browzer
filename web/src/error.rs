use std::sync::{self, mpsc};
use thiserror::Error;

// ----- defining custom error enum for `WebServer`
#[derive(Debug, Error)]
pub enum WebServerError {
    #[error("Stream flush error: {0}")]
    StreamFlushError(String),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Empty HTTP request")]
    EmptyRequestError,
}

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
