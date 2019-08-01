// use super::*;
use super::send_method;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    prelude::*,
    types::{
        shipping,
        value::{self, Seq, ShippingQueryId},
    },
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
    shipping_query_id: ShippingQueryId<'a>,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Seq<'a, value::Ref<'a, shipping::Option<'a>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<value::String<'a>>,
}

impl<'a, C> AnswerShippingQuery<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        shipping_query_id: impl Into<ShippingQueryId<'a>>,
        result: Result<
            Seq<'a, value::Ref<'a, shipping::Option<'a>>>,
            value::String<'a>,
        >,
    ) -> Self {
        let ok = result.is_ok();
        let (shipping_options, error_message) = match result {
            Ok(shipping_options) => (Some(shipping_options), None),
            Err(error_message) => (None, Some(error_message)),
        };

        Self {
            client,
            token,
            shipping_query_id: shipping_query_id.into(),
            ok,
            shipping_options,
            error_message,
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
