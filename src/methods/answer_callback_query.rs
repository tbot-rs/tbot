use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{callback, parameters::CallbackAction},
};
use serde::Serialize;

/// Answers a callback query.
///
/// Reflects the [`answerCallbackQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answercallbackquery
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerCallbackQuery<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    callback_query_id: callback::query::id::Ref<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u64>,
}

impl<'a> AnswerCallbackQuery<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        callback_query_id: callback::query::id::Ref<'a>,
        action: Option<CallbackAction<'a>>,
    ) -> Self {
        Self {
            bot,
            callback_query_id,
            text: action.as_ref().and_then(|x| x.to_text()),
            show_alert: action.as_ref().and_then(|x| x.to_show_alert()),
            url: action.as_ref().and_then(|x| x.to_url()),
            cache_time: None,
        }
    }

    /// Configures the amount of time (in seconds) for which the answer may be
    /// cached. Reflects the `cache_time` parameter.
    pub const fn cache_time(mut self, time: u64) -> Self {
        self.cache_time = Some(time);
        self
    }
}

impl AnswerCallbackQuery<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "answerCallbackQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
