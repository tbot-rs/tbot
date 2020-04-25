use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{game::HighScore, inline_message_id, user},
};
use serde::Serialize;

/// Gets an excerpt from the high score table of a game sent via the inline
/// mode.
///
/// Reflects the [`getGameHighScores`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetInlineGameHighScores<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    user_id: user::Id,
    inline_message_id: inline_message_id::Ref<'a>,
}

impl<'a> GetInlineGameHighScores<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            inline_message_id,
        }
    }
}

impl GetInlineGameHighScores<'_> {
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
