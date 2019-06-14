use super::*;
use crate::internal::Client;
use parameters::NotificationState;

/// Represents the [`forwardMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#forwardmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct ForwardMessage<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
    from_chat_id: types::ChatId<'a>,
    message_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a, C> ForwardMessage<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Configures `disable_notification`.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }
}

impl<C> IntoFuture for ForwardMessage<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "forwardMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
