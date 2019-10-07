use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    prelude::{Future, IntoFuture},
    Token,
};
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
    token: Token,
    sticker: &'a str,
}

impl<'a, C> DeleteStickerFromSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
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

impl<C: Connector> IntoFuture for DeleteStickerFromSet<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "deleteStickerFromSet",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
