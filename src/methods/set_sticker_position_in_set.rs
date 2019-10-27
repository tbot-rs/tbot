use super::send_method;
use crate::{connectors::Connector, errors, internal::Client, token};
use serde::Serialize;

/// Changes a sticker's position in a sticker set.
///
/// Reflects the [`setStickerPositionInSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setstickerpositioninset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetStickerPositionInSet<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    sticker: &'a str,
    position: u32,
}

impl<'a, C> SetStickerPositionInSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
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

impl<C: Connector> SetStickerPositionInSet<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
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
