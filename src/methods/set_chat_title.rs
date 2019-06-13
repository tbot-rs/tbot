use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`setChatTitle`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchattitle
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatTitle<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
    title: &'a str,
}

impl<'a, C> SetChatTitle<'a, C> {
    /// Constructs a new `SetChatTitle`.
    pub fn new(
        client: Arc<Client<C>>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        title: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            title,
        }
    }
}

impl<C> IntoFuture for SetChatTitle<'_, C>
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
                "setChatTitle",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
