use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`kickChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#kickchatmember
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct KickChatMember<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
}

impl<'a, C> KickChatMember<'a, C> {
    pub(crate) fn new(
        client: Arc<Client<C>>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
        }
    }

    /// Configures `until_date`.
    pub fn until_date(mut self, date: i64) -> Self {
        self.until_date = Some(date);
        self
    }
}

impl<C> IntoFuture for KickChatMember<'_, C>
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
                "kickChatMember",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on successы
        )
    }
}
