use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::{inline_message_id, keyboard::inline},
};

/// Edits the inline keyboard of a message sent via the inline mode.
///
/// Reflects the [`editMessageReplyMarkup`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineReplyMarkup<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    inline_message_id: inline_message_id::Ref<'a>,
    reply_markup: inline::Keyboard<'a>,
}

impl<'a, C> EditInlineReplyMarkup<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: inline_message_id::Ref<'a>,
        reply_markup: inline::Keyboard<'a>,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
            reply_markup,
        }
    }
}

impl<C: Connector> IntoFuture for EditInlineReplyMarkup<'_, C> {
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
