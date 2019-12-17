use crate::errors::MethodCall;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::time::Elapsed;

/// Represents possible errors that a webhook server may return.
#[derive(Debug)]
pub enum HttpWebhook {
    /// An error during setting the webhook.
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
                 with an error: {}",
                error,
            ),
        }
    }
}

impl Error for HttpWebhook {}

impl HttpWebhook {
    /// Checks if `self` is `SetWebhook`.
    pub fn is_set_webhook(&self) -> bool {
        match self {
            Self::SetWebhook(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SetWebhookTimeout`.
    pub fn is_set_webhook_timeout(&self) -> bool {
        match self {
            Self::SetWebhookTimeout(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Server`.
    pub fn is_server(&self) -> bool {
        match self {
            Self::Server(..) => true,
            _ => false,
        }
    }
}

impl From<MethodCall> for HttpWebhook {
    fn from(error: MethodCall) -> Self {
        Self::SetWebhook(error)
    }
}

impl From<Elapsed> for HttpWebhook {
    fn from(timeout: Elapsed) -> Self {
        Self::SetWebhookTimeout(timeout)
    }
}

impl From<hyper::Error> for HttpWebhook {
    fn from(error: hyper::Error) -> Self {
        Self::Server(error)
    }
}
