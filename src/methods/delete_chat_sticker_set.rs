use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Deletes a chat's sticker set.
///
/// Reflects the [`deleteChatStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#deletechatstickerset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteChatStickerSet<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a> DeleteChatStickerSet<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
        }
    }
}

impl DeleteChatStickerSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "deleteChatStickerSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
