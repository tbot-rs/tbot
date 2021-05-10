use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        chat,
        keyboard::inline,
        message::{self, Message},
        parameters::Invoice,
    },
};
use serde::Serialize;

/// Sends an invoice.
///
/// Reflects the [`sendInvoice`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendinvoice
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendInvoice<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: chat::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard>,
    #[serde(flatten)]
    invoice: Invoice,
}

impl<'a> SendInvoice<'a> {
    #[allow(clippy::too_many_arguments)] // I know, brother
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl Into<chat::Id>,
        invoice: Invoice,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            invoice,
            start_parameter: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this invoice is sent in reply to.
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
    #[allow(clippy::missing_const_for_fn)]
    pub fn reply_markup(mut self, markup: inline::Keyboard) -> Self {
        self.reply_markup = Some(markup);
        self
    }

    /// Configures unique deep-linking parameter for "Pay button" to redirect.
    /// Reflects the `start_parameter` parameter.
    pub fn start_parameter(
        mut self,
        start_parameter: impl Into<String>,
    ) -> Self {
        self.start_parameter = Some(start_parameter.into());
        self
    }
}

impl SendInvoice<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "sendInvoice",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
