use super::*;
use crate::internal::Client;

/// Represents the [`sendChatAction`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendchataction
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendChatAction<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
    action: types::ChatAction,
}

impl<'a, C> SendChatAction<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        action: types::ChatAction,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            action,
        }
    }
}

impl<C> IntoFuture for SendChatAction<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "sendChatAction",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
