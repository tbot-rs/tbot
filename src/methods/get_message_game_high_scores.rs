use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        game::HighScore,
        message,
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};

/// Gets an excerpt from the high score table of a game sent by the bot itself.
///
/// Reflects the [`getGameHighScores`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMessageGameHighScores<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: user::Id,
    chat_id: ChatId<'a>,
    message_id: message::Id,
}

impl<'a, C> GetMessageGameHighScores<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
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

impl<C: Connector> GetMessageGameHighScores<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<HighScore>, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "getGameHighScores",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
