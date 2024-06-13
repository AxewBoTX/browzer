//! This module defines custom error types used throughout the web server framework.

// External crate imports
use thiserror::Error;

// Standard library imports
use std::sync::{self, mpsc};

/// Custom error type for the `ThreadPool`.
#[derive(Debug, Error)]
pub enum ThreadPoolError {
    /// Error when the receiver lock is poisoned.
    #[error("Receiver lock error: {0}")]
    ReceiverLockError(String),

    /// Error when receiving a message from the channel.
    #[error("Receive error: {0}")]
    ReceiveError(#[from] mpsc::RecvError),

    /// Error when sending a message through the channel.
    #[error("Send error: {0}")]
    SendError(String),
}

/// Implement conversion from `PoisonError` to `ThreadPoolError::ReceiverLockError`.
impl<T> From<sync::PoisonError<T>> for ThreadPoolError {
    fn from(err: sync::PoisonError<T>) -> Self {
        ThreadPoolError::ReceiverLockError(err.to_string())
    }
}

/// Custom error type for the `Request`.
#[derive(Debug, Error)]
pub enum RequestError {
    /// Error for an invalid request line.
    #[error("Invalid request line: {0}")]
    InvalidRequestLineError(String),

    /// Error for an empty HTTP request.
    #[error("Empty HTTP request")]
    EmptyRequestError,
}

/// Custom error type for the `WebServer`.
#[derive(Debug, Error)]
pub enum WebServerError {
    /// Error when flushing a stream.
    #[error("Stream flush error: {0}")]
    StreamFlushError(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    /// Error when parsing a request.
    #[error("Request parse error: {0}")]
    RequestParseError(RequestError),

    /// Internal server error.
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

/// Custom error type for the `WebRouter`
#[derive(Debug, Error)]
pub enum WebRouterError {
    /// Error while formatting a path
    #[error("Error while formatting a path: {0}")]
    PathFormatError(String),
}
