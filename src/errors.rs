//! Types representing errors.

use crate::types::chat;
use hyper::StatusCode;

/// Represents possible errors whic may occur while downloading a file.
#[derive(Debug)]
pub enum Download {
    /// The provided file had the `path` field set to `None`.
    NoPath,
    /// A network error.
    Network(hyper::Error),
    /// Telegram returned a different from 200 status code.
    InvalidStatusCode(StatusCode),
}

/// Represents possible errors that may happen during a method call.
#[derive(Debug)]
pub enum MethodCall {
    /// A network error.
    Network(hyper::Error),
    /// Bots API is likely to be down.
    TelegramOutOfService,
    /// An error returned in response.
    RequestError {
        /// A human-readable description of the error.
        description: String,
        /// The error code for this error.
        error_code: u16,
        /// The group moved to a supergroup with the following ID.
        migrate_to_chat_id: Option<chat::Id>,
        /// The bot exceeded flood threshold. You can make another request
        /// after the following amount of seconds.
        retry_after: Option<u64>,
    },
}
