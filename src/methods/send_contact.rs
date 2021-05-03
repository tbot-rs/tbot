use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;
use std::borrow::Cow;

/// Sends a contact.
///
/// Reflects the [`sendContact`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendcontact
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendContact<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    phone_number: Cow<'a, str>,
    first_name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendContact<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        phone_number: impl Into<Cow<'a, str>>,
        first_name: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    /// Configures the last name of the contact.
    /// Reflects the `last_name` parameter.
    pub fn last_name(mut self, last_name: impl Into<Cow<'a, str>>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Configures a VCard for the contact. Reflects the `vcard` parameter.
    #[allow(clippy::doc_markdown)] // no, I don't need to put VCard in backticks
    pub fn vcard(mut self, vcard: impl Into<Cow<'a, str>>) -> Self {
        self.vcard = Some(vcard.into());
        self
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this contact is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub const fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures whether this message should be sent even
    /// if the replied-to message is not found.
    /// Reflects the `allow_sending_without_reply` parameter.
    pub const fn allow_sending_without_reply(mut self) -> Self {
        self.allow_sending_without_reply = true;
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

impl SendContact<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "sendContact",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
