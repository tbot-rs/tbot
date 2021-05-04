use super::call_method;
use crate::{bot::InnerBot, errors};
use serde::Serialize;

/// Deletes a sticker from a sticker set.
///
/// Reflects the [`deleteStickerFromSet`][docs] method
///
/// [docs]: https://core.telegram.org/bots/api#deletestickerfromset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteStickerFromSet<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    sticker: String,
}

impl<'a> DeleteStickerFromSet<'a> {
    pub(crate) fn new(bot: &'a InnerBot, sticker: impl Into<String>) -> Self {
        Self {
            bot,
            sticker: sticker.into(),
        }
    }
}

impl DeleteStickerFromSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "deleteStickerFromSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
