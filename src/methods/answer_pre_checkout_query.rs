use super::call_method;
use crate::{connectors::Client, errors, token, types::pre_checkout_query};
use serde::Serialize;
use std::borrow::Cow;

/// Answers a pre-checkout query.
///
/// Reflects the [`answerPreCheckoutQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answerprecheckoutquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerPreCheckoutQuery<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    pre_checkout_query_id: pre_checkout_query::id::Ref<'a>,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<Cow<'a, str>>,
}

impl<'a> AnswerPreCheckoutQuery<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        pre_checkout_query_id: pre_checkout_query::id::Ref<'a>,
        result: Result<(), impl Into<Cow<'a, str>>>,
    ) -> Self {
        Self {
            client,
            token,
            pre_checkout_query_id,
            ok: result.is_ok(),
            error_message: result.err().map(|e| e.into()),
        }
    }
}

impl AnswerPreCheckoutQuery<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "answerPreCheckoutQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
