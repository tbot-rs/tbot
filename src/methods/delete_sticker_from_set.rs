use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::value,
};

/// Represents the [`deleteStickerFromSet`][docs] method
///
/// [docs]: https://core.telegram.org/bots/api#deletestickerfromset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteStickerFromSet<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    sticker: value::String<'a>,
}

impl<'a, C> DeleteStickerFromSet<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        sticker: impl Into<value::String<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            sticker: sticker.into(),
        }
    }
}

impl<C> IntoFuture for DeleteStickerFromSet<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
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
