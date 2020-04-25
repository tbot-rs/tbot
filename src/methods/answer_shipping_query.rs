use super::send_method;
use crate::{connectors::Client, errors, token, types::shipping};
use serde::Serialize;

/// Answers a shipping query.
///
/// Reflects the [`answerShippingQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answershippingquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerShippingQuery<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    shipping_query_id: shipping::query::id::Ref<'a>,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<&'a [shipping::Option<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<&'a str>,
}

impl<'a> AnswerShippingQuery<'a> {
    pub(crate) fn new(
        client: &'a Client,
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

impl AnswerShippingQuery<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
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
