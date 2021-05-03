use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard::inline,
        message,
        parameters::{ChatId, ImplicitChatId},
        Poll,
    },
};
use serde::Serialize;

/// Stops a poll.
///
/// Reflects the [`stopPoll`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#stoppoll
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopPoll<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> StopPoll<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl StopPoll<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Poll, errors::MethodCall> {
        call_method(
            self.bot,
            "stopPoll",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
