use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        keyboard::inline,
        value::{InlineMessageId, Ref},
    },
};

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
    inline_message_id: InlineMessageId<'a>,
    reply_markup: Ref<'a, inline::Keyboard<'a>>,
}

impl<'a, C> EditInlineReplyMarkup<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        reply_markup: impl Into<Ref<'a, inline::Keyboard<'a>>>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id: inline_message_id.into(),
            reply_markup: reply_markup.into(),
        }
    }
}

impl<C> IntoFuture for EditInlineReplyMarkup<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

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
