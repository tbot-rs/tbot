use crate::errors::MethodCall;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::timer::timeout::Elapsed;

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
            HttpWebhook::SetWebhook(error) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook` \
                 failed with an error: {}",
                error,
            ),
            HttpWebhook::SetWebhookTimeout(timeout) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook`
                timed out: {}",
                timeout,
            ),
            HttpWebhook::Server(error) => write!(
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
            HttpWebhook::SetWebhook(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SetWebhookTimeout`.
    pub fn is_set_webhook_timeout(&self) -> bool {
        match self {
            HttpWebhook::SetWebhookTimeout(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Server`.
    pub fn is_server(&self) -> bool {
        match self {
            HttpWebhook::Server(..) => true,
            _ => false,
        }
    }
}

impl From<MethodCall> for HttpWebhook {
    fn from(error: MethodCall) -> Self {
        HttpWebhook::SetWebhook(error)
    }
}

impl From<Elapsed> for HttpWebhook {
    fn from(timeout: Elapsed) -> Self {
        HttpWebhook::SetWebhookTimeout(timeout)
    }
}

impl From<hyper::Error> for HttpWebhook {
    fn from(error: hyper::Error) -> Self {
        HttpWebhook::Server(error)
    }
}
