use super::MethodCall;
use is_macro::Is;
use tokio::time::Elapsed;

/// Represents possible errors that may happen during preparation of the
/// polling event loop.
#[derive(Debug, Is)]
pub enum PollingSetup {
    /// Calling `DeleteWebhook` resulted in an error.
    DeleteWebhook(MethodCall),
    /// Calling `DeleteWebhook` timed out.
    DeleteWebhookTimeout(Elapsed),
    /// Calling the `setMyCommands` method resulted in an error.
    SetMyCommands(MethodCall),
    /// Calling the `setMyCommands` method timed out.
    SetMyCommandsTimeout(Elapsed),
}

impl From<MethodCall> for PollingSetup {
    #[must_use]
    fn from(error: MethodCall) -> Self {
        Self::DeleteWebhook(error)
    }
}

impl From<Elapsed> for PollingSetup {
    #[must_use]
    fn from(error: Elapsed) -> Self {
        Self::DeleteWebhookTimeout(error)
    }
}
