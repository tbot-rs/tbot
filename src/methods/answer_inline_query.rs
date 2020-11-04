use super::call_method;
use crate::{connectors::Client, errors, token, types::inline_query};
use serde::Serialize;

/// Answers an inline query.
///
/// Reflects the [`answerInlineQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answerinlinequery
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerInlineQuery<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    inline_query_id: inline_query::id::Ref<'a>,
    results: &'a [inline_query::Result<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_parameter: Option<&'a str>,
}

impl<'a> AnswerInlineQuery<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        inline_query_id: inline_query::id::Ref<'a>,
        results: &'a [inline_query::Result<'a>],
    ) -> Self {
        Self {
            client,
            token,
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    /// Configures the amount of time (in seconds) for which the answer may be
    /// cached. Reflects the `cache_time` parameter.
    pub fn cache_time(mut self, time: u64) -> Self {
        self.cache_time = Some(time);
        self
    }

    /// Configures whether the result may be cached only for the user who sent
    /// the query. Reflects the `is_personal` parameter.
    pub fn personal(mut self, is_personal: bool) -> Self {
        self.is_personal = Some(is_personal);
        self
    }

    /// Configures the offset to be sent in the next query.
    /// Reflects the `next_offset` parameter.
    pub fn next_offset(mut self, offset: &'a str) -> Self {
        self.next_offset = Some(offset);
        self
    }

    /// Configures a button that switches the user to the private chat
    /// with your bot. Reflects the `switch_pm_text` and `switch_pm_parameter`
    /// parameters respectively.
    pub fn switch_pm(mut self, text: &'a str, parameter: &'a str) -> Self {
        self.switch_pm_text = Some(text);
        self.switch_pm_parameter = Some(parameter);
        self
    }
}

impl AnswerInlineQuery<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.client,
            self.token,
            "answerInlineQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
