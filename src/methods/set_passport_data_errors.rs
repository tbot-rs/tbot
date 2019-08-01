use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        passport, user,
        value::{Ref, Seq},
    },
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
    errors: Seq<'a, Ref<'a, passport::element::Error<'a>>>,
}

impl<'a, C> SetPassportDataErrors<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
        errors: impl Into<Seq<'a, Ref<'a, passport::element::Error<'a>>>>,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            errors: errors.into(),
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
    type Error = errors::MethodCall;

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
