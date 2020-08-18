use super::call_method;
use crate::{bot::InnerBot, errors, types::sticker};
use serde::Serialize;
use std::borrow::Cow;

/// Gets a sticker set by its name.
///
/// Reflects the [`getStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getstickerset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetStickerSet<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    name: Cow<'a, str>,
}

impl<'a> GetStickerSet<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        name: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            bot,
            name: name.into(),
        }
    }
}

impl GetStickerSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<sticker::Set, errors::MethodCall> {
        call_method(
            self.bot,
            "getStickerSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
