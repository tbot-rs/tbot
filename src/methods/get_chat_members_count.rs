use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`getChatMembersCount`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmemberscount
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMembersCount<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
}

impl<'a, C> GetChatMembersCount<'a, C> {
    /// Constructs a new `GetChatMembersCount`.
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

impl<C> IntoFuture for GetChatMembersCount<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = u32;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.client,
            &self.token,
            "getChatMembersCount",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
