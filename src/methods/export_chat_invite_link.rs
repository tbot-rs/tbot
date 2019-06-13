use super::*;
use crate::internal::Client;

/// Represents the [`exportChatInviteLink`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#exportchatinvitelink
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct ExportChatInviteLink<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: types::ChatId<'a>,
}

impl<'a, C> ExportChatInviteLink<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
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

impl<C> IntoFuture for ExportChatInviteLink<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = String;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.client,
            &self.token,
            "exportChatInviteLink",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
