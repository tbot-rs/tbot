use super::MethodCall;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::time::Elapsed;

/// Represents possible errors that an HTTPS webhook server may return.
#[derive(Debug)]
#[must_use]
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

impl HttpsWebhook {
    /// Checks if `self` is `SetWebhook`.
    #[must_use]
    pub fn is_set_webhook(&self) -> bool {
        match self {
            Self::SetWebhook(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SetWebhookTimeout`.
    #[must_use]
    pub fn is_set_webhook_timeout(&self) -> bool {
        match self {
            Self::SetWebhookTimeout(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Tls`.
    #[must_use]
    pub fn is_tls(&self) -> bool {
        match self {
            Self::Tls(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Bind`.
    #[must_use]
    pub fn is_bind(&self) -> bool {
        match self {
            Self::Bind(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Server`.
    #[must_use]
    pub fn is_server(&self) -> bool {
        match self {
            Self::Server(..) => true,
            _ => false,
        }
    }
}

impl Display for HttpsWebhook {
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
                "The webhook event loop failed because a call to `setWebhook` \
                 timed out: {}",
                timeout,
            ),
            Self::Tls(timeout) => write!(
                formatter,
                "The webhook event loop failed because TLS initialization \
                 failed with an error: {}",
                timeout,
            ),
            Self::Bind(timeout) => write!(
                formatter,
                "The webhook event loop failed because failed to bind to a \
                 port: {}",
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
