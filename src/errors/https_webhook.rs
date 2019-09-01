use crate::errors::MethodCall;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::timer::timeout;

type Timeout = timeout::Error<MethodCall>;

/// Represents possible errors that an HTTPS webhook server may return.
#[derive(Debug)]
pub enum HttpsWebhook {
    /// An error during setting the webhook.
    SetWebhook(Timeout),
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
            HttpsWebhook::SetWebhook(timeout) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook` \
                 failed with an error: {}",
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
