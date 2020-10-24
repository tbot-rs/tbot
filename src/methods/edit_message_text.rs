use super::call_method;
#[allow(deprecated)]
use crate::{
    connectors::Client,
    errors, token,
    types::{
        keyboard::inline,
        message::{self, Message},
        parameters::{
            ChatId, ImplicitChatId, ParseMode, Text, WebPagePreviewState,
        },
    },
};
use serde::Serialize;

/// Edits the text of a message sent by the bot itself.
///
/// Reflects the [`editMessageText`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagetext
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageText<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditMessageText<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        text: impl Into<Text<'a>>,
    ) -> Self {
        let text = text.into();

        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            text: text.text,
            parse_mode: text.parse_mode,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Configures if a preview for the first link in the message should be
    /// shown. Reflects the `disable_web_page_preview` parameter.
    pub fn is_web_page_preview_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_web_page_preview_disabled` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn web_page_preview(self, state: WebPagePreviewState) -> Self {
        self.is_web_page_preview_disabled(state.is_disabled())
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditMessageText<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "editMessageText",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
