use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
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
    bot: &'a InnerBot,
    user_id: user::Id,
    inline_message_id: inline_message_id::Ref<'a>,
}

impl<'a> GetInlineGameHighScores<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            bot,
            user_id,
            inline_message_id,
        }
    }
}

impl GetInlineGameHighScores<'_> {
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
