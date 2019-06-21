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
    OutOfService,
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

impl Download {
    /// Checks if `self` is `NoPath`.
    pub fn is_no_path(&self) -> bool {
        match self {
            Download::NoPath => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Network`.
    pub fn is_network(&self) -> bool {
        match self {
            Download::Network(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `InvalidStatusCode`.
    pub fn is_invalid_status_code(&self) -> bool {
        match self {
            Download::InvalidStatusCode(..) => true,
            _ => false,
        }
    }
}

impl MethodCall {
    /// Checks if `self` is `Network`.
    pub fn is_network(&self) -> bool {
        match self {
            MethodCall::Network(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `OutOfService`.
    pub fn is_out_of_service(&self) -> bool {
        match self {
            MethodCall::OutOfService => true,
            _ => false,
        }
    }

    /// Checks if `self` is `RequestError`.
    pub fn is_request_error(&self) -> bool {
        match self {
            MethodCall::RequestError {
                ..
            } => true,
            _ => false,
        }
    }
}
