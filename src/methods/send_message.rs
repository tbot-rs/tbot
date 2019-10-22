use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        keyboard, message,
        parameters::{
            ChatId, ImplicitChatId, NotificationState, ParseMode, Text,
            WebPagePreviewState,
        },
    },
};

/// Sends a text message.
///
/// Reflects the [`sendMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendMessage<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendMessage<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        text: impl Into<Text<'a>>,
    ) -> Self {
        let text = text.into();

        Self {
            client,
            token,
            chat_id: chat_id.into(),
            text: text.text,
            parse_mode: text.parse_mode,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures if a preview for the first link in the message should be
    /// shown. Reflects the `disable_web_page_preview` parameter.
    pub fn web_page_preview(mut self, state: WebPagePreviewState) -> Self {
        self.disable_web_page_preview = Some(state.is_disabled());
        self
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures which message this text message is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C: Connector> SendMessage<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<types::Message, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "sendMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
