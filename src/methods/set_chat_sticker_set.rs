use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::parameters::{ChatId, ImplicitChatId},
    token,
};
use serde::Serialize;

/// Sets a group's sticker set.
///
/// Reflects the [`setChatStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatstickerset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatStickerSet<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    sticker_set_name: &'a str,
}

impl<'a, C> SetChatStickerSet<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        sticker_set_name: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            sticker_set_name,
        }
    }
}

impl<C: Connector> SetChatStickerSet<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
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
