use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Deletes a chat's photo.
///
/// Reflects the [`deleteChatPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#deletechatphoto
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteChatPhoto<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
}

impl<'a> DeleteChatPhoto<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl DeleteChatPhoto<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "deleteChatPhoto",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
