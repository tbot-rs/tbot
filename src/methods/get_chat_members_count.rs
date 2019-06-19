use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::parameters::{ChatId, ImplicitChatId},
};

/// Represents the [`getChatMembersCount`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmemberscount
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMembersCount<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
}

impl<'a, C> GetChatMembersCount<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
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
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = u32;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getChatMembersCount",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
