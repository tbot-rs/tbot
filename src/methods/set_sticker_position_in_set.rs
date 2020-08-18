use super::call_method;
use crate::{bot::InnerBot, errors};
use serde::Serialize;

/// Changes a sticker's position in a sticker set.
///
/// Reflects the [`setStickerPositionInSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setstickerpositioninset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetStickerPositionInSet<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    sticker: &'a str,
    position: u32,
}

impl<'a> SetStickerPositionInSet<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        sticker: &'a str,
        position: u32,
    ) -> Self {
        Self {
            bot,
            sticker,
            position,
        }
    }
}

impl SetStickerPositionInSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setStickerPositionInSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
