use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        keyboard::inline,
        parameters::{ParseMode, Text, WebPagePreviewState},
        value::{self, InlineMessageId, Ref},
    },
};

/// Represents the [`editMessageText`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagetext
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineText<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    inline_message_id: InlineMessageId<'a>,
    text: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<Ref<'a, inline::Keyboard<'a>>>,
}

impl<'a, C> EditInlineText<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: impl Into<InlineMessageId<'a>>,
        text: impl Into<Text<'a>>,
    ) -> Self {
        let Text {
            text,
            parse_mode,
        } = text.into();

        Self {
            client,
            token,
            inline_message_id: inline_message_id.into(),
            text,
            parse_mode,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Configures `disable_web_page_preview`.
    pub fn web_page_preview(mut self, state: WebPagePreviewState) -> Self {
        self.disable_web_page_preview = Some(state.is_disabled());
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<Ref<'a, inline::Keyboard<'a>>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C> IntoFuture for EditInlineText<'_, C>
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
                "editMessageText",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
