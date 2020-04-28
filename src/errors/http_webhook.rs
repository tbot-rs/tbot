use crate::errors::MethodCall;
use is_macro::Is;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::time::Elapsed;

/// Represents possible errors that a webhook server may return.
#[derive(Debug, Is)]
pub enum HttpWebhook {
    /// An error while setting the webhook.
    SetWebhook(MethodCall),
    /// Calling the `setWebhook` method timed out.
    SetWebhookTimeout(Elapsed),
    /// An error while running the server.
    Server(hyper::Error),
}

impl Display for HttpWebhook {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::SetWebhook(error) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook` \
                 failed with an error: {}",
                error,
            ),
            Self::SetWebhookTimeout(timeout) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook`
                timed out: {}",
                timeout,
            ),
            Self::Server(error) => write!(
                formatter,
                "The webhook event loop failed because the server returned \
                 an error: {}",
                error,
            ),
        }
    }
}

impl Error for HttpWebhook {}

impl From<MethodCall> for HttpWebhook {
    #[must_use]
    fn from(error: MethodCall) -> Self {
        Self::SetWebhook(error)
    }
}

impl From<Elapsed> for HttpWebhook {
    #[must_use]
    fn from(timeout: Elapsed) -> Self {
        Self::SetWebhookTimeout(timeout)
    }
}

impl From<hyper::Error> for HttpWebhook {
    #[must_use]
    fn from(error: hyper::Error) -> Self {
        Self::Server(error)
    }
}
