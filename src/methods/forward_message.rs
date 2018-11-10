use super::*;

/// Representation of the [`forwardMessage`] method.
///
/// [`forwardMessage`]: https://core.telegram.org/bots/api#forwardmessage
#[derive(Serialize)]
pub struct ForwardMessage<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    from_chat_id: &'a types::ChatId<'a>,
    message_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a> ForwardMessage<'a> {
    /// Creates a new `ForwardMessage`.
    #[must_use]
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        from_chat_id: &'a types::ChatId,
        message_id: u64,
    ) -> ForwardMessage<'a> {
        ForwardMessage {
            token,
            chat_id: chat_id.into(),
            from_chat_id,
            message_id,
            disable_notification: None,
        }
    }

    /// Sets `the disable_notification` field to `Some(is_disabled)`.
    #[must_use]
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method::<types::raw::Message>(
            self.token,
            "forwardMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}
