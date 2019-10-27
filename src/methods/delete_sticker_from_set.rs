use super::send_method;
use crate::{connectors::Connector, errors, internal::Client, token};
use serde::Serialize;

/// Deletes a sticker from a sticker set.
///
/// Reflects the [`deleteStickerFromSet`][docs] method
///
/// [docs]: https://core.telegram.org/bots/api#deletestickerfromset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteStickerFromSet<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    sticker: &'a str,
}

impl<'a, C> DeleteStickerFromSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        sticker: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            sticker,
        }
    }
}

impl<C: Connector> DeleteStickerFromSet<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            self.token,
            "deleteStickerFromSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
