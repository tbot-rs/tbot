use tokio::timer::timeout;
use crate::errors::MethodCall;

type Timeout = timeout::Error<MethodCall>;

/// Represents possible errors that a webhook server may return.
#[derive(Debug)]
pub enum Webhook {
    /// An error during setting the webhook.
    SetWebhook(Timeout),
    /// An error while running the server.
    Server(hyper::Error),
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

impl From<Timeout> for Webhook {
    fn from(error: Timeout) -> Self {
        Webhook::SetWebhook(error)
    }
}

impl From<hyper::Error> for Webhook {
    fn from(error: hyper::Error) -> Self {
        Webhook::Server(error)
    }
}
