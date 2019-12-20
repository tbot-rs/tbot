use crate::types::chat;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Represents possible errors that may happen during a method call.
#[derive(Debug)]
pub enum MethodCall {
    /// A network error.
    Network(hyper::Error),
    /// Bots API is likely to be down.
    OutOfService,
    /// Failed to parse the response.
    Parse {
        /// The response which failed to parse.
        response: Vec<u8>,
        /// The error which parsing failed with.
        error: serde_json::Error,
    },
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

impl MethodCall {
    /// Checks if `self` is `Network`.
    #[must_use]
    pub fn is_network(&self) -> bool {
        match self {
            Self::Network(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `OutOfService`.
    #[must_use]
    pub fn is_out_of_service(&self) -> bool {
        match self {
            Self::OutOfService => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Parse`.
    #[must_use]
    pub fn is_parse(&self) -> bool {
        match self {
            Self::Parse { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `RequestError`.
    #[must_use]
    pub fn is_request_error(&self) -> bool {
        match self {
            Self::RequestError { .. } => true,
            _ => false,
        }
    }
}

impl Display for MethodCall {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Network(error) => write!(
                formatter,
                "A method call failed because of a network error: {}",
                error,
            ),
            Self::OutOfService => write!(
                formatter,
                "A method call failed because Telegram is out of service.",
            ),
            Self::Parse { response, error } => write!(
                formatter,
                "A method call failed because `tbot` failed to parse the \
                response.\n\n\

                The response was: {response:?}\n\
                The error was: {error}",
                response = response,
                error = error,
            ),
            Self::RequestError {
                description,
                error_code,
                migrate_to_chat_id,
                retry_after,
            } => write!(
                formatter,
                "A method call failed because Telegram responded with an error \
                {error_code} `{description}`. Additional information:\n\n\

                - migrate_to_chat_id: {chat_id:?}\n\
                - retry_after: {retry_after:?}",
                error_code = error_code,
                description = description,
                chat_id = migrate_to_chat_id,
                retry_after = retry_after,
            ),
        }
    }
}

impl Error for MethodCall {}

impl From<hyper::Error> for MethodCall {
    #[must_use]
    fn from(error: hyper::Error) -> Self {
        Self::Network(error)
    }
}
