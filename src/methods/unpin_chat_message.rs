use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`unpinChatMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#unpinchatmessage
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UnpinChatMessage<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
}

impl<'a, C> UnpinChatMessage<'a, C> {
    /// Constructs a new `UnpinChatMessage`.
    pub fn new(
        client: Arc<Client<C>>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
        }
    }
}

impl<C> IntoFuture for UnpinChatMessage<'_, C>
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
                &self.client,
                &self.token,
                "unpinChatMessage",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
