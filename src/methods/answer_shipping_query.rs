// use super::*;
use super::send_method;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    prelude::*,
    types::shipping,
    Token,
};
use serde::Serialize;

/// Represents the [`answerShippingQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answershippingquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerShippingQuery<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    shipping_query_id: shipping::query::id::Ref<'a>,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<&'a [shipping::Option<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<&'a str>,
}

impl<'a, C> AnswerShippingQuery<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        shipping_query_id: shipping::query::id::Ref<'a>,
        result: Result<&'a [shipping::Option<'a>], &'a str>,
    ) -> Self {
        Self {
            client,
            token,
            shipping_query_id,
            ok: result.is_ok(),
            shipping_options: result.ok(),
            error_message: result.err(),
        }
    }
}

impl<C> IntoFuture for AnswerShippingQuery<'_, C>
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
                "answerShippingQuery",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
