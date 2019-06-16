use super::*;
use crate::{
    internal::Client,
    types::{
        keyboard::inline,
        parameters::{ChatId, ParseMode, WebPagePreviewState},
    },
};

/// Represents the [`editMessageText`][docs] method for chat messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagetext
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageText<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    message_id: u32,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> EditMessageText<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<ChatId<'a>>,
        message_id: u32,
        text: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, mode: ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Configures `disable_web_page_preview`.
    pub fn web_page_preview(mut self, state: WebPagePreviewState) -> Self {
        self.disable_web_page_preview = Some(state.is_disabled());
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<C> IntoFuture for EditMessageText<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "editMessageText",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
