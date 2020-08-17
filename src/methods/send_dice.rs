use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        dice::Kind,
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
        Message,
    },
};
use serde::Serialize;

/// Sends a dice.
///
/// Reflects the [`sendDice`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#senddice
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendDice<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    #[serde(rename = "emoji")]
    kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendDice<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            kind: Kind::Dice,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Сonfigures the dice's kind. Reflects the `emoji` parameter.
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub fn kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
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
    pub const fn reply_to_message_id(mut self, id: message::Id) -> Self {
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

impl SendDice<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "sendDice",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
