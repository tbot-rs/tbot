use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{InlineMessageId, user},
};
use serde::Serialize;

/// Sets a user's new high score in a game sent via the inline mode.
///
/// Reflects the [`setGameScore`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setgamescore
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetInlineGameScore<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    user_id: user::Id,
    score: u32,
    inline_message_id: InlineMessageId<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
}

impl<'a> SetInlineGameScore<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        inline_message_id: InlineMessageId<'a>,
        user_id: user::Id,
        score: u32,
    ) -> Self {
        Self {
            bot,
            user_id,
            score,
            inline_message_id,
            force: None,
            disable_edit_message: None,
        }
    }

    /// Configures if the score may go down. Reflects the `force` parameter.
    pub const fn force(mut self, is_forced: bool) -> Self {
        self.force = Some(is_forced);
        self
    }

    /// Configures if the message should not be edited immediately.
    /// Reflects the `disable_edit_message` parameter.
    pub const fn disable_edit_message(mut self, is_disabled: bool) -> Self {
        self.disable_edit_message = Some(is_disabled);
        self
    }
}

impl SetInlineGameScore<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setGameScore",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
