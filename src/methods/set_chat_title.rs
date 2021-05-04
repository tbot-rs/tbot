use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Sets a group's title.
///
/// Reflects the [`setChatTitle`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchattitle
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatTitle<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    title: String,
}

impl<'a> SetChatTitle<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        title: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }
}

impl SetChatTitle<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setChatTitle",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
