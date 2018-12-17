use super::*;

/// Representation of the [`sendChatAction`] method.
///
/// [`sendChatAction`]: https://core.telegram.org/bots/api#sendchataction
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendChatAction<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    action: types::ChatAction,
}

impl<'a> SendChatAction<'a> {
    /// Constructs a new `SendChatAction`.
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        action: types::ChatAction,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            action,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "sendChatAction",
            None,
            serde_json::to_vec(&self).unwrap(),
            // It returns only `true` if suceess, handling it is meaningless.
        )
        .map(|_| ())
    }
}
