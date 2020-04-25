use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
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
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> StopPoll<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl StopPoll<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Poll, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "stopPoll",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
