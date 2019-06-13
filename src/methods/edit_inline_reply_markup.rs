use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`editMessageReplyMarkup`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineReplyMarkup<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    inline_message_id: &'a str,
    reply_markup: types::InlineKeyboard<'a>,
}

impl<'a, C> EditInlineReplyMarkup<'a, C> {
    /// Constructs a new `EditInlineReplyMarkup`.
    pub const fn new(
        client: Arc<Client<C>>,
        token: Token,
        inline_message_id: &'a str,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
            reply_markup,
        }
    }
}

impl<C> IntoFuture for EditInlineReplyMarkup<'_, C>
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
                "editMessageReplyMarkup",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
