use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
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
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    user_id: user::Id,
    chat_id: ChatId<'a>,
    message_id: message::Id,
}

impl<'a> GetMessageGameHighScores<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl GetMessageGameHighScores<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<HighScore>, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "getGameHighScores",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
