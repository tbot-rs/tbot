// use super::*;
use super::send_method;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    prelude::*,
    types::value::{self, PreCheckoutQueryId},
    Token,
};
use serde::Serialize;

/// Represents the [`answerPreCheckoutQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answerprecheckoutquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerPreCheckoutQuery<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    pre_checkout_query_id: PreCheckoutQueryId<'a>,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<value::String<'a>>,
}

impl<'a, C> AnswerPreCheckoutQuery<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        pre_checkout_query_id: impl Into<PreCheckoutQueryId<'a>>,
        result: Result<(), value::String<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok: result.is_ok(),
            error_message: result.err(),
        }
    }
}

impl<C> IntoFuture for AnswerPreCheckoutQuery<'_, C>
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
                "answerPreCheckoutQuery",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
