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
    callback_query_id: callback::query::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u64>,
}

impl<'a> AnswerCallbackQuery<'a> {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: &'a InnerBot,
        callback_query_id: callback::query::Id,
        action: Option<CallbackAction>,
    ) -> Self {
        match action {
            None => Self {
                bot,
                callback_query_id,
                text: None,
                show_alert: None,
                url: None,
                cache_time: None,
            },
            Some(CallbackAction::Url(url)) => Self {
                bot,
                callback_query_id,
                text: None,
                show_alert: None,
                url: Some(url),
                cache_time: None,
            },
            Some(CallbackAction::Text(text, show_alert)) => Self {
                bot,
                callback_query_id,
                text: Some(text),
                show_alert: Some(show_alert),
                url: None,
                cache_time: None,
            },
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
