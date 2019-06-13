use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`deleteMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#deletemessage
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteMessage<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
    message_id: u32,
}

impl<'a, C> DeleteMessage<'a, C> {
    pub(crate) fn new(
        client: Arc<Client<C>>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl<C> IntoFuture for DeleteMessage<'_, C>
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
                "deleteMessage",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
