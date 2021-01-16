use super::MethodCall;
use is_macro::Is;
use tokio::time::error::Elapsed;

/// Represents possible errors that may happen during the polling event loop.
#[derive(Debug, Is)]
pub enum Polling {
    /// Calling `GetUpdates` resulted in an error.
    Fetching(MethodCall),
    /// Calling `GetUpdates` timed out.
    Timeout(Elapsed),
}

impl From<MethodCall> for Polling {
    #[must_use]
    fn from(error: MethodCall) -> Self {
        Self::Fetching(error)
    }
}

impl From<Elapsed> for Polling {
    #[must_use]
    fn from(error: Elapsed) -> Self {
        Self::Timeout(error)
    }
}
