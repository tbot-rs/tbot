use super::*;

/// Representation of the [`getMe`] method.
///
/// [`getMe`]: https://core.telegram.org/bots/api#getme
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMe<'a> {
    token: &'a str,
}

impl<'a> GetMe<'a> {
    /// Constructs a new `GetMe`.
    pub fn new<'b: 'a>(token: &'b str) -> Self {
        Self {
            token,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::User, Error = DeliveryError> {
        send_method::<types::User>(self.token, "getMe", None, Vec::new())
    }
}
