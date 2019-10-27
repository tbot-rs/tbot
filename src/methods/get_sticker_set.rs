use super::send_method;
use crate::{
    connectors::Connector, errors, internal::Client, types::sticker, token,
};
use serde::Serialize;

/// Gets a sticker set by its name.
///
/// Reflects the [`getStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getstickerset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetStickerSet<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    name: &'a str,
}

impl<'a, C> GetStickerSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        name: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            name,
        }
    }
}

impl<C: Connector> GetStickerSet<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<sticker::Set, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "getStickerSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
