use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;
use std::borrow::Cow;

/// Sets a group's sticker set.
///
/// Reflects the [`setChatStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatstickerset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatStickerSet<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    sticker_set_name: Cow<'a, str>,
}

impl<'a> SetChatStickerSet<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        sticker_set_name: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }
}

impl SetChatStickerSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "setChatStickerSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
