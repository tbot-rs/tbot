use super::*;
use crate::internal::Client;

/// Represents the [`setStickerPositionInSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setstickerpositioninset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetStickerPositionInSet<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    sticker: &'a str,
    position: u32,
}

impl<'a, C> SetStickerPositionInSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
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

impl<C> IntoFuture for SetStickerPositionInSet<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "setStickerPositionInSet",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
