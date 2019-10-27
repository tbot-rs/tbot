use super::send_method;
use crate::{
    connectors::Connector, errors, internal::Client, types::pre_checkout_query,
    Token,
};
use serde::Serialize;

/// Answers a pre-checkout query.
///
/// Reflects the [`answerPreCheckoutQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answerprecheckoutquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerPreCheckoutQuery<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    pre_checkout_query_id: pre_checkout_query::id::Ref<'a>,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<&'a str>,
}

impl<'a, C> AnswerPreCheckoutQuery<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        pre_checkout_query_id: pre_checkout_query::id::Ref<'a>,
        result: Result<(), &'a str>,
    ) -> Self {
        Self {
            client,
            token,
            pre_checkout_query_id,
            ok: result.is_ok(),
            error_message: result.err(),
        }
    }
}

impl<C: Connector> AnswerPreCheckoutQuery<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            &self.token,
            "answerPreCheckoutQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
