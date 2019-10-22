use super::MethodCall;
use tokio::timer::timeout::Elapsed;

/// Reperesents possible errors that may happen during preparation of the
/// polling event loop.
#[derive(Debug)]
pub enum PollingSetup {
    /// Calling `DeleteWebhook` resulted in an error.
    DeleteWebhook(MethodCall),
    /// Calling `DeleteWebhook` timed out.
    DeleteWebhookTimeout(Elapsed),
}

impl PollingSetup {
    /// Checks if `self` is `DeleteWebhook`.
    pub fn is_delete_webhook(&self) -> bool {
        match self {
            PollingSetup::DeleteWebhook(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `DeleteWebhookTimeout`.
    pub fn is_delete_webhook_timeout(&self) -> bool {
        match self {
            PollingSetup::DeleteWebhookTimeout(..) => true,
            _ => false,
        }
    }
}
impl From<MethodCall> for PollingSetup {
    fn from(error: MethodCall) -> Self {
        PollingSetup::DeleteWebhook(error)
    }
}

impl From<Elapsed> for PollingSetup {
    fn from(error: Elapsed) -> Self {
        PollingSetup::DeleteWebhookTimeout(error)
    }
}
