use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::{passport, user},
};

/// Represents the [`setPassportDataErrors`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setpassportdataerrors
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetPassportDataErrors<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: user::Id,
    errors: &'a [passport::element::Error<'a>],
}

impl<'a, C> SetPassportDataErrors<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
        errors: &'a [passport::element::Error<'a>],
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            errors,
        }
    }
}

impl<C> IntoFuture for SetPassportDataErrors<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "setPassportDataErrors",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()),
        )
    }
}
