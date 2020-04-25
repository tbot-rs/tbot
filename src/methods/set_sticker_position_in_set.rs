use super::send_method;
use crate::{connectors::Client, errors, token};
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
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    sticker: &'a str,
    position: u32,
}

impl<'a> SetStickerPositionInSet<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        sticker: &'a str,
        position: u32,
    ) -> Self {
        Self {
            client,
            token,
            sticker,
            position,
        }
    }
}

impl SetStickerPositionInSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            self.client,
            self.token,
            "setStickerPositionInSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
