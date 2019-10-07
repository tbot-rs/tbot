use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::parameters::{ChatId, ImplicitChatId},
};

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
    token: Token,
    chat_id: ChatId<'a>,
    sticker_set_name: &'a str,
}

impl<'a, C> SetChatStickerSet<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
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

impl<C: Connector> IntoFuture for SetChatStickerSet<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "setChatStickerSet",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
