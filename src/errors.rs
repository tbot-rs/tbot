use hyper::StatusCode;
use crate::types::chat;

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
pub enum DeliveryError {
    /// Failed to send the request.
    NetworkError(hyper::Error),
    /// Telegram Bots API is likely to be down.
    TelegramOutOfService,
    /// Failed to parse the response from Telegram. It's likely to be a bug
    /// in `tbot`, so feel free to fill an issue on [our GitLab][issues].
    ///
    /// [issues]: https://gitlab.com/snejugal/tbot/issues
    InvalidResponse(serde_json::error::Error),
    /// Telegram returned an error in response.
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
