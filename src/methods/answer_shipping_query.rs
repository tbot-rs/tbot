use super::call_method;
use crate::{bot::InnerBot, errors, types::shipping};
use serde::Serialize;
use std::borrow::Cow;

/// Answers a shipping query.
///
/// Reflects the [`answerShippingQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answershippingquery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerShippingQuery<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    shipping_query_id: shipping::query::Id,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Cow<'a, [shipping::Option]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl<'a> AnswerShippingQuery<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        shipping_query_id: shipping::query::Id,
        result: Result<
            impl Into<Cow<'a, [shipping::Option]>>,
            impl Into<String>,
        >,
    ) -> Self {
        if result.is_ok() {
            Self {
                bot,
                shipping_query_id,
                ok: true,
                shipping_options: result.ok().map(Into::into),
                error_message: None,
            }
        } else {
            Self {
                bot,
                shipping_query_id,
                ok: false,
                shipping_options: None,
                error_message: result.err().map(Into::into),
            }
        }
    }
}

impl AnswerShippingQuery<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "answerShippingQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
