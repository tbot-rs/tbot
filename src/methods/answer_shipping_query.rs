use super::send_method;
use crate::{
    connectors::Connector, errors, internal::Client, types::shipping, token,
};
use serde::Serialize;

/// Answers a shipping query.
///
/// Reflects the [`answerShippingQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answershippingquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerShippingQuery<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
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
        token: token::Ref<'a>,
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

impl<C: Connector> AnswerShippingQuery<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            self.token,
            "answerShippingQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
