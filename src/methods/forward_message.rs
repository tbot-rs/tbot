use super::*;

/// Representation of the [`forwardMessage`] method.
///
/// [`forwardMessage`]: https://core.telegram.org/bots/api#forwardmessage
#[derive(Serialize)]
pub struct ForwardMessage<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: &'a types::ChatId<'a>,
    from_chat_id: &'a types::ChatId<'a>,
    message_id: u64,
    disable_notification: Option<bool>,
}

impl<'a> ForwardMessage<'a> {
    /// Creates a new `ForwardMessage`.
    #[must_use]
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: &'a types::ChatId,
        from_chat_id: &'a types::ChatId,
        message_id: u64,
    ) -> ForwardMessage<'a> {
        ForwardMessage {
            token,
            chat_id,
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
    pub fn get_request(
        &self,
    ) -> impl Future<Item = types::Message, Error = DeliveryError> {
        send_method::<types::Message>(
            self.token,
            "forwardMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}
