use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{inline_message_id, user},
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
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    user_id: user::Id,
    score: u32,
    inline_message_id: inline_message_id::Ref<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
}

impl<'a> SetInlineGameScore<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        inline_message_id: inline_message_id::Ref<'a>,
        user_id: user::Id,
        score: u32,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            score,
            inline_message_id,
            force: None,
            disable_edit_message: None,
        }
    }

    /// Configures if the score may go down. Reflects the `force` parameter.
    pub fn is_forced(mut self, is_forced: bool) -> Self {
        self.force = Some(is_forced);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `is_forced`"
    )]
    pub fn force(self, is_forced: bool) -> Self {
        self.is_forced(is_forced)
    }

    /// Configures if the message should not be edited immediately.
    /// Reflects the `disable_edit_message` parameter.
    pub fn is_editing_message_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_edit_message = Some(is_disabled);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `is_editing_message_disabled`"
    )]
    pub fn disable_edit_message(self, is_disabled: bool) -> Self {
        self.is_editing_message_disabled(is_disabled)
    }
}

impl SetInlineGameScore<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "setGameScore",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
