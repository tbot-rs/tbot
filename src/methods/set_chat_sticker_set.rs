use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Sets a group's sticker set.
///
/// Reflects the [`setChatStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatstickerset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatStickerSet<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    sticker_set_name: &'a str,
}

impl<'a> SetChatStickerSet<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        sticker_set_name: &'a str,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            sticker_set_name,
        }
    }
}

impl SetChatStickerSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setChatStickerSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
