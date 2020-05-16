use super::call_method;
use crate::{connectors::Client, errors, token};
use serde::Serialize;
use std::borrow::Cow;

/// Deletes a sticker from a sticker set.
///
/// Reflects the [`deleteStickerFromSet`][docs] method
///
/// [docs]: https://core.telegram.org/bots/api#deletestickerfromset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteStickerFromSet<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    sticker: Cow<'a, str>,
}

impl<'a> DeleteStickerFromSet<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        sticker: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            client,
            token,
            sticker: sticker.into(),
        }
    }
}

impl DeleteStickerFromSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
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
