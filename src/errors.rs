//! Types representing errors.

use crate::types::chat;
use hyper::StatusCode;
use tokio::timer::timeout;

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

/// Represents possible errors that a webhook server may return.
#[derive(Debug)]
pub enum Webhook {
    /// An error during setting the webhook.
    SetWebhook(timeout::Error<MethodCall>),
    /// An error while running the server.
    Server(hyper::Error),
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

impl Webhook {
    /// Checks if `self` is `SetWebhook`.
    pub fn is_set_webhook(&self) -> bool {
        match self {
            Webhook::SetWebhook(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Server`.
    pub fn is_server(&self) -> bool {
        match self {
            Webhook::Server(..) => true,
            _ => false,
        }
    }
}
