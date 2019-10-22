use super::MethodCall;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::timer::timeout::Elapsed;

/// Represents possible errors that an HTTPS webhook server may return.
#[derive(Debug)]
pub enum HttpsWebhook {
    /// An error during setting the webhook.
    SetWebhook(MethodCall),
    /// Calling the `setWebhook` method timed out.
    SetWebhookTimeout(Elapsed),
    /// An error during initializing TLS.
    Tls(native_tls::Error),
    /// An error during port binding.
    Bind(std::io::Error),
    /// An error while running the server.
    Server(hyper::Error),
}

impl Display for HttpsWebhook {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            HttpsWebhook::SetWebhook(error) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook` \
                 failed with an error: {}",
                error,
            ),
            HttpsWebhook::SetWebhookTimeout(timeout) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook` \
                 timed out: {}",
                timeout,
            ),
            HttpsWebhook::Tls(timeout) => write!(
                formatter,
                "The webhook event loop failed because TLS initialization \
                 failed with an error: {}",
                timeout,
            ),
            HttpsWebhook::Bind(timeout) => write!(
                formatter,
                "The webhook event loop failed because failed to bind to a \
                 port: {}",
                timeout,
            ),
            HttpsWebhook::Server(error) => write!(
                formatter,
                "The webhook event loop failed because the server returned \
                 with an error: {}",
                error,
            ),
        }
    }
}

impl Error for HttpsWebhook {}

impl From<MethodCall> for HttpsWebhook {
    fn from(error: MethodCall) -> Self {
        Self::SetWebhook(error)
    }
}

impl From<Elapsed> for HttpsWebhook {
    fn from(error: Elapsed) -> Self {
        Self::SetWebhookTimeout(error)
    }
}

impl From<native_tls::Error> for HttpsWebhook {
    fn from(error: native_tls::Error) -> Self {
        Self::Tls(error)
    }
}

impl From<std::io::Error> for HttpsWebhook {
    fn from(error: std::io::Error) -> Self {
        Self::Bind(error)
    }
}

impl From<hyper::Error> for HttpsWebhook {
    fn from(error: hyper::Error) -> Self {
        Self::Server(error)
    }
}
