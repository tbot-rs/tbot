use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`deleteStickerFromSet`][docs] method
///
/// [docs]: https://core.telegram.org/bots/api#deletestickerfromset
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteStickerFromSet<'a, C> {
    #[serde(skip)]
    client: Arc<Client<C>>,
    #[serde(skip)]
    token: Token,
    sticker: &'a str,
}

impl<'a, C> DeleteStickerFromSet<'a, C> {
    /// Constructs a new `DeleteStickerFromSet`.
    pub const fn new(
        client: Arc<Client<C>>,
        token: Token,
        sticker: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            sticker,
        }
    }
}

impl<C> IntoFuture for DeleteStickerFromSet<'_, C>
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
                &self.client,
                &self.token,
                "deleteStickerFromSet",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
