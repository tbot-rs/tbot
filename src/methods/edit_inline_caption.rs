use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{inline_message_id, keyboard::inline, parameters::ParseMode},
};
/// Represents the [`editMessageCaption`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagecaption
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineCaption<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    inline_message_id: inline_message_id::Ref<'a>,
    caption: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> EditInlineCaption<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: inline_message_id::Ref<'a>,
        caption: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            inline_message_id,
            caption,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, mode: ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<C> IntoFuture for EditInlineCaption<'_, C>
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
                "editMessageCaption",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
