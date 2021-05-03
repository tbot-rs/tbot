use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        game::HighScore,
        message,
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};
use serde::Serialize;

/// Gets an excerpt from the high score table of a game sent by the bot itself.
///
/// Reflects the [`getGameHighScores`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMessageGameHighScores<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    user_id: user::Id,
    chat_id: ChatId,
    message_id: message::Id,
}

impl<'a> GetMessageGameHighScores<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        user_id: user::Id,
    ) -> Self {
        Self {
            bot,
            user_id,
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl GetMessageGameHighScores<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<HighScore>, errors::MethodCall> {
        call_method(
            self.bot,
            "getGameHighScores",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
