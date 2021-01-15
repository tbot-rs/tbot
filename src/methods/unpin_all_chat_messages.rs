use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Unpins all messages in a chat.
///
/// Reflects the [`unpinAllChatMessages`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#unpinallchatmessages
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UnpinAllChatMessages<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
}

impl<'a> UnpinAllChatMessages<'a> {
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

impl UnpinAllChatMessages<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "unpinAllChatMessages",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
