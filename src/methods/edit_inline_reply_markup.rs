use super::*;
use crate::internal::Client;

/// Represents the [`editMessageReplyMarkup`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineReplyMarkup<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    inline_message_id: &'a str,
    reply_markup: types::InlineKeyboard<'a>,
}

impl<'a, C> EditInlineReplyMarkup<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
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
                self.client,
                &self.token,
                "editMessageReplyMarkup",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
