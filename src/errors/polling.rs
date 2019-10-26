use super::MethodCall;
use tokio::timer::timeout::Elapsed;

/// Represent possible errors that may happen during the polling event loop.
#[derive(Debug)]
pub enum Polling {
    /// Calling `GetUpdates` resulted in an error.
    Fetching(MethodCall),
    /// Calling `GetUpdates` timed out.
    Timeout(Elapsed),
}

impl Polling {
    /// Checks if `self` is `Fetching`.
    pub fn is_fetching(&self) -> bool {
        match self {
            Self::Fetching(..) => true,
            _ => false,
        }
    }
    /// Checks if `self` is `Timeout`.
    pub fn is_timeout(&self) -> bool {
        match self {
            Self::Timeout(..) => true,
            _ => false,
        }
    }
}

impl From<MethodCall> for Polling {
    fn from(error: MethodCall) -> Self {
        Self::Fetching(error)
    }
}

impl From<Elapsed> for Polling {
    fn from(error: Elapsed) -> Self {
        Self::Timeout(error)
    }
}
